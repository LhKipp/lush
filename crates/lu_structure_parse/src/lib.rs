use std::path::PathBuf;

use lu_error::util::Outcome;
use lu_interpreter_structs::*;
use lu_syntax::{
    ast::{FnStmtNode, SourceFileNode, StrctStmtNode, UseStmtNode},
    AstNode,
};

pub fn source_file_and_put_into_scope(
    source_node: &SourceFileNode,
    source_f_name: PathBuf,
    scope: &mut Scope<Variable>,
) -> Outcome<()> {
    assert!(scope.cur_frame().get_tag() == &ScopeFrameTag::GlobalFrame);
    let funcs_parent_frame_id = scope.get_cur_frame_id();

    let (sourced_file, errors) = source_structures_from(source_node).split();
    let (_, frame) = scope.push_frame(ScopeFrameTag::SourceFileFrame(source_f_name));

    for func in sourced_file.funcs {
        func.set_parent_frame_id(funcs_parent_frame_id);
        frame.insert_var(Variable::new_func(func.rced()));
    }
    for strct in sourced_file.strcts {
        frame.insert_var(Variable::new_strct(strct));
    }
    for _ in sourced_file.use_paths {
        todo!();
    }

    Outcome::new((), errors)
}

struct SourcedFile {
    strcts: Vec<Strct>,
    funcs: Vec<Function>,
    use_paths: Vec<UsePath>,
}

fn source_structures_from(source_node: &SourceFileNode) -> Outcome<SourcedFile> {
    let block = source_node.block().unwrap();

    // TODO source variables
    let (use_paths, err1) = block
        .use_stmts()
        .map(|use_stmt| source_use_stmt(&use_stmt))
        .collect::<Outcome<_>>()
        .split();
    let (funcs, err2) = block
        .fn_stmts()
        .map(|fn_stmt| source_fn_stmt(&fn_stmt))
        .collect::<Outcome<_>>()
        .split();
    let (strcts, err3) = block
        .struct_stmts()
        .map(|strct_stmt| source_struct_stmt(&strct_stmt))
        .collect::<Outcome<_>>()
        .split();
    let errors = err1
        .into_iter()
        .chain(err2.into_iter())
        .chain(err3.into_iter())
        .collect();

    Outcome::new(
        SourcedFile {
            strcts,
            funcs,
            use_paths,
        },
        errors,
    )
}

fn source_fn_stmt(fn_stmt: &FnStmtNode) -> Outcome<Function> {
    let name = fn_stmt.name().unwrap_or("".to_string());
    // Source the signature (either user provided or default)
    let (sign, errors) = Signature::from_sign_and_stmt(fn_stmt.signature(), fn_stmt.decl_item());

    Outcome::new(Function::new(name, sign, fn_stmt.clone()), errors)
}

fn source_struct_stmt(struct_stmt: &StrctStmtNode) -> Outcome<Strct> {
    let name = struct_stmt.name().unwrap_or("".to_string());

    // Source the struct fields (either user provided or default)
    let fields: Outcome<Vec<StrctField>> = struct_stmt
        .fields()
        .map(|field| StrctField::from_node(&field).into())
        .collect();

    fields.map(|fields| Strct::new(name, fields, struct_stmt.to_item()))
}

fn source_use_stmt(use_stmt: &UseStmtNode) -> Outcome<UsePath> {
    Outcome::ok(UsePath::new(use_stmt.parts(), use_stmt.to_item()))
}
