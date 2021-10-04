#![allow(unused_imports)]
use crate::{eval_function, Command, ScopeFrameId};
use lu_parser::grammar::SourceFileRule;
use lu_text_util::SourceCode;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};
use walkdir::WalkDir;

use log::debug;
use lu_error::{EvalErr, FsErr, LuResult, SourceCodeItem};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{
        BlockStmtNode, FnStmtNode, IfStmtNode, LuTypeNode, SignatureNode, StatementElement,
        StrctStmtNode,
    },
    ast::{ConditionElement, IfBlockNode, SourceFileNode, UseStmtNode},
    AstElement, AstNode, AstToken, Parse,
};
use lu_syntax_elements::{
    constants::{IN_ARG_NAME, RET_ARG_NAME},
    BlockType,
};
use lu_value::Value;

use crate::{
    resolve::{Resolve, ResolveArg, Resolver},
    visit_arg::VisitArg,
    ArgDecl, ArgSignature, EvalArg, Evaluable, FlagSignature, Function, Interpreter, ScopeFrameTag,
    Signature, Strct, StrctField, ValueType, Variable,
};

impl Resolve for BlockStmtNode {
    fn do_resolve_dependant_names(&self, args: &[ResolveArg], resolver: &mut Resolver) {
        let source_f_path = match args.get(0) {
            Some(ResolveArg::Arg(VisitArg::SourceFileBlock(f))) => f,
            _ => unreachable!("Passing of BlockType as first arg is required"),
        };
        {
            let mut l_scope = resolver.scope.lock();
            l_scope.push_frame(ScopeFrameTag::SourceFileFrame(source_f_path.clone()));
        }

        // For each struct/fn_stmt we have to do:
        // 1. Put the fn with signature into the current scope
        // 2. resolve all dependant names within the fn_stmt // TODO undone and yeah that should be
        //    worth a check
        // Step 2 should be done in sequence with resolution of the source file block, so as to have
        // global vars in scope
        // ```lu
        // let x = 1
        // mut_x # Call can happen before sourcing fn stmt. Therefore source fn stmts before
        // fn mut_x
        //      $x = 3 # $x refers to global x
        // end
        // ```
        for use_stmt in self.use_stmts() {
            source_use_stmt(use_stmt, resolver);
        }

        for fn_stmt in self.fn_stmts() {
            source_fn_stmt(&fn_stmt, resolver);
        }
        for struct_stmt in self.struct_stmts() {
            source_struct_stmt(&struct_stmt, resolver);
        }

        // TODO source variables

        debug!(
            "Scope after resolving block stmt: {}",
            resolver.scope.lock().fmt_as_string()
        );
        // No deletion of the source file frame, so following steps can use it
    }
}

fn source_use_stmt(use_stmt: UseStmtNode, resolver: &mut Resolver) {
    // Save old source_file_frame_id and go to parent
    let orig_source_f_frame_id = resolver.scope.lock().get_cur_frame_id();
    resolver.scope.lock().select_parent_frame();

    // if use_stmt.is_std_path() {
    //     // lu_
    // } else {
    // }

    let path_to_source = resolver.config.plugin_dir.join(use_stmt.path_as_path_buf());
    debug!("sourcing plugin: {:?}", path_to_source);

    for entry in WalkDir::new(path_to_source).into_iter() {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    debug!("sourcing plugin entry: {:?}", entry.path());
                    if let Some(code) =
                        resolver.ok_or_record(SourceCode::new_file(entry.into_path()))
                    {
                        let new_source_f_id = source_file(code, resolver);
                        resolver
                            .scope
                            .lock()
                            .use_stmts
                            .insert(orig_source_f_frame_id, new_source_f_id);
                    }
                }
            }
            Err(e) => resolver.push_err(FsErr::Message(e.to_string()).into()),
        }
    }

    resolver
        .scope
        .lock()
        .set_cur_frame_id(orig_source_f_frame_id);
}

/// Returns NodeId of sourced file
/// Leaves scope cur_frame_id unchanged
fn source_file(code: SourceCode, resolver: &mut Resolver) -> ScopeFrameId {
    assert!(resolver
        .scope
        .lock()
        .get_cur_frame_tag()
        .as_global_frame()
        .is_some());
    let f_name = code.path.clone();
    let parse = Parse::rule(code, &SourceFileRule {});
    // Better add the parse before resolving it
    resolver.parses.push(parse);
    // Get ref to just pushed parse
    let parse = resolver
        .parses
        .last()
        .unwrap()
        .cast::<SourceFileNode>()
        .unwrap();

    // push a new frame for the file to source
    parse.resolve_dependant_names_with_args(
        &[ResolveArg::Arg(VisitArg::SourceFilePath(f_name))],
        resolver,
    );
    let new_source_f_id = resolver.scope.lock().get_cur_frame_id();
    resolver.scope.lock().select_parent_frame();
    assert!(resolver
        .scope
        .lock()
        .get_cur_frame_tag()
        .as_global_frame()
        .is_some());
    new_source_f_id
}

fn source_fn_stmt(fn_stmt: &FnStmtNode, resolver: &mut Resolver) {
    let name = fn_stmt.name().unwrap_or("".to_string());

    // Source the signature (either user provided or default)
    let (sign, errs) = Signature::from_sign_and_stmt(
        fn_stmt.signature(),
        fn_stmt.decl_item(),
        &resolver.scope.lock(),
    );
    resolver.get_mut_errors().extend(errs);

    let parent_frame_id = resolver.scope.lock().get_cur_frame_id();
    let func = Function::new(
        name,
        sign,
        fn_stmt.clone(),
        parent_frame_id,
        Box::new(eval_function),
    )
    .boxed();

    resolver
        .scope
        .lock()
        .cur_mut_frame()
        .insert_var(Variable::new_func(func));
}

fn source_struct_stmt(struct_stmt: &StrctStmtNode, resolver: &mut Resolver) {
    let name = struct_stmt.name().unwrap_or("".to_string());

    // Source the struct fields (either user provided or default)
    let fields: Vec<StrctField> = struct_stmt
        .fields()
        .map(|field| {
            let (field, errs) = StrctField::from_node(&field, &resolver.scope.lock());
            resolver.push_errs(errs);
            field
        })
        .collect();
    let strct = Strct::new(name, fields, struct_stmt.to_item());

    resolver.scope.lock().cur_mut_frame().insert(
        strct.name.clone(),
        Variable::new_struct(strct, struct_stmt.clone()),
    );
}
