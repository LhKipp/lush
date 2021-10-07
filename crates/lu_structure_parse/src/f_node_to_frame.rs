use lu_error::util::Outcome;
use lu_interpreter_structs::*;
use lu_syntax::{
    ast::{FnStmtNode, SourceFileNode, StrctStmtNode, UseStmtNode},
    AstNode,
};

/// Convert a SourceFileNode to a ScopeFrame representation.
/// No struct-types will be resolved (they are left as ValueType::StructName)
pub fn source_node_to_scope_frame(
    source_node: &SourceFileNode,
    source_f_name: UsePath,
) -> Outcome<ScopeFrame<Variable>> {
    let sourced_file = source_structures_from(source_node, source_f_name.clone());
    sourced_file.map(|sourced_file| {
        let mut frame = ScopeFrame::new(ScopeFrameTag::SourceFileFrame {
            id: source_f_name,
            use_paths: sourced_file.use_paths,
        });

        for func in sourced_file.funcs {
            frame.insert_var(Variable::new_func(func.rced()));
        }
        for strct in sourced_file.strcts {
            frame.insert_var(Variable::new_strct(strct));
        }
        frame
    })
}

struct SourcedFile {
    strcts: Vec<Strct>,
    funcs: Vec<Function>,
    use_paths: Vec<UsePath>,
}

fn source_structures_from(
    source_node: &SourceFileNode,
    source_node_id: UsePath,
) -> Outcome<SourcedFile> {
    let block = source_node.block().unwrap();

    // TODO source variables
    let (use_paths, err1) = block
        .use_stmts()
        .map(|use_stmt| source_use_stmt(&use_stmt))
        .collect::<Outcome<_>>()
        .split();
    let (funcs, err2) = block
        .fn_stmts()
        .map(|fn_stmt| source_fn_stmt(&fn_stmt, source_node_id.clone()))
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

fn source_fn_stmt(fn_stmt: &FnStmtNode, source_file_id: UsePath) -> Outcome<Function> {
    let name = fn_stmt.name().unwrap_or("".to_string());
    // Source the signature (either user provided or default)
    let (sign, errors) = Signature::from_sign_and_stmt(fn_stmt.signature(), fn_stmt.decl_item());

    Outcome::new(
        Function::new(name, sign, fn_stmt.clone(), source_file_id),
        errors,
    )
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
    let ty = if use_stmt.is_std_path() {
        UsePathVariant::StdPath
    } else if use_stmt.is_plugin_path() {
        UsePathVariant::PluginPath
    } else {
        assert!(use_stmt.is_file_path());
        UsePathVariant::FilePath
    };
    Outcome::ok(UsePath::new(use_stmt.parts(), ty, use_stmt.to_item()))
}
