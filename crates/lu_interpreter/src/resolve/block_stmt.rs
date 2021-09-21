#![allow(unused_imports)]
use std::rc::Rc;

use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_pipeline_stage::ErrorContainer;
use lu_syntax::{
    ast::{BlockStmtNode, FnStmtNode, IfStmtNode, LuTypeNode, SignatureNode, StatementElement},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use lu_syntax_elements::{
    constants::{IN_ARG_NAME, RET_ARG_NAME},
    BlockType,
};
use lu_value::Value;

use crate::{
    callable::Decl,
    resolve::{Resolve, ResolveArg, Resolver},
    visit_arg::VisitArg,
    ArgSignature, EvalArg, Evaluable, FlagSignature, Function, Interpreter, ScopeFrameTag,
    Signature, ValueType, Variable, ARG_VAR_NAME,
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

        // For each fn_stmt we have to do:
        // 1. Put the fn with signature into the current scope
        // 2. resolve all dependant names within the fn_stmt
        // Step 2 is done in sequence with resolution of the source file block, so as to have
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

        // TODO source variables

        // No deletion of the source file frame, so following steps can use it
    }
}

fn source_fn_stmt(fn_stmt: &FnStmtNode, resolver: &mut Resolver) {
    let name = fn_stmt.name().unwrap_or("".to_string());

    // Source the signature (either user provided or default)
    let sign = if let Some(sign_node) = fn_stmt.signature() {
        source_signature(&sign_node, fn_stmt, resolver)
    } else {
        Signature::default_signature(fn_stmt)
    };

    let parent_frame_id = resolver.scope.lock().get_cur_frame_id();
    let func = Function::new(name, sign, fn_stmt.clone(), parent_frame_id);

    resolver
        .scope
        .lock()
        .cur_mut_frame()
        .insert(func.name.clone(), Variable::new_func(func, fn_stmt.clone()));
}

fn source_signature(
    sign_node: &SignatureNode,
    fn_stmt: &FnStmtNode,
    resolver: &mut Resolver,
) -> Signature {
    let (in_ty, err) = ArgSignature::from_node(sign_node.in_arg(), IN_ARG_NAME, fn_stmt);
    resolver.record_option(err);
    let (ret_ty, err) = ArgSignature::from_node(sign_node.ret_arg(), RET_ARG_NAME, fn_stmt);
    resolver.record_option(err);
    let args: Vec<ArgSignature> = sign_node
        .args()
        .iter()
        .map(|arg_node| -> ArgSignature {
            let arg_name = arg_node.name();
            let ty = arg_node
                .type_()
                .map(|ty_node| get_ty_of_node(&ty_node, resolver))
                .unwrap_or(ValueType::Unspecified);
            ArgSignature::new(arg_name, ty, arg_node.clone().into())
        })
        .collect();
    let flags = sign_node
        .flags()
        .map(|flag_node| -> FlagSignature {
            let long_name = flag_node.long_name();
            let short_name = flag_node.short_name();
            let ty = flag_node
                .type_()
                .map(|ty_node| get_ty_of_node(&ty_node, resolver))
                .unwrap_or(ValueType::Unspecified);
            FlagSignature::new(long_name, short_name, ty, Some(flag_node))
        })
        .collect();
    let var_arg = sign_node.var_arg().map(|var_arg_node| {
        let name = var_arg_node.name();
        let ty = var_arg_node
            .type_()
            .map(|ty_node| get_ty_of_node(&ty_node, resolver))
            .unwrap_or(ValueType::Any);
        ArgSignature::new(name, ty, var_arg_node.into())
    });
    Signature::new(args, var_arg, flags, in_ty, ret_ty)
}

fn get_ty_of_node(ty_node: &LuTypeNode, resolver: &mut Resolver) -> ValueType {
    let ty = ValueType::from_node(&ty_node.into_type());
    resolver.ok_or_record(ty).unwrap_or(ValueType::Error)
}
