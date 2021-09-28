#![allow(unused_imports)]
use std::rc::Rc;

use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{
        BlockStmtNode, FnStmtNode, IfStmtNode, LuTypeNode, SignatureNode, StatementElement,
        StrctStmtNode,
    },
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstNode, AstToken,
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
    Signature, Strct, StrctField, ValueType, Variable, ARG_VAR_NAME,
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
        for fn_stmt in self.fn_stmts() {
            source_fn_stmt(&fn_stmt, resolver);
        }
        for struct_stmt in self.struct_stmts() {
            source_struct_stmt(&struct_stmt, resolver);
        }

        // TODO source variables

        // No deletion of the source file frame, so following steps can use it
    }
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
    let func = Function::new(name, sign, fn_stmt.clone(), parent_frame_id);

    resolver
        .scope
        .lock()
        .cur_mut_frame()
        .insert(func.name.clone(), Variable::new_func(func, fn_stmt.clone()));
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
