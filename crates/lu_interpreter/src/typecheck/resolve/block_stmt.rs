#![allow(unused_imports)]
use std::rc::Rc;

use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{BlockStmtNode, FnStmtNode, IfStmtNode, LuTypeNode, SignatureNode, StatementElement},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use lu_syntax_elements::BlockType;
use lu_value::Value;

use crate::{
    typecheck::{ResoElem, Resolve, ResolveArg, Resolver, ValueType},
    ArgSignature, EvalArg, Evaluable, FlagSignature, Function, Interpreter, ScopeFrameTag,
    Signature, VarArgSignature, Variable, ARG_VAR_NAME,
};

use super::TyFunction;

impl Resolve for BlockStmtNode {
    fn do_resolve_dependant_names(&self, args: &[ResolveArg], resolver: &mut Resolver) {
        assert!(!args.is_empty(), "Passing of BlockType is required");
        let b_type = match &args[0] {
            ResolveArg::BlockTypeArg(t) => t,
            _ => unreachable!("Passing of BlockType as first arg is required"),
        };
        resolver.scope.push_frame(b_type.clone().into());

        if b_type == &BlockType::SourceFileBlock {
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
        }
    }
}

fn source_fn_stmt(fn_stmt: &FnStmtNode, resolver: &mut Resolver) {
    let parent_frame_id = resolver.scope.get_cur_frame_id();

    resolver.scope.push_frame(ScopeFrameTag::FnFrame);
    let name = fn_stmt.name().unwrap_or("".to_string());

    // Source the signature (either user provided or default)
    let sign = if let Some(sign_node) = fn_stmt.signature() {
        source_signature(&sign_node, resolver)
    } else {
        let args = (0..10)
            .map(|i| ArgSignature::new(ARG_VAR_NAME.to_string() + &i.to_string(), None, true))
            .collect();
        Signature::new(args, None, vec![], None, None)
    };

    let func = Function::new(name, sign, fn_stmt.clone(), parent_frame_id);
    let ty_func = TyFunction::from_func(func, resolver);

    resolver
        .scope
        .cur_mut_frame()
        .insert(ty_func.func.name.clone(), ResoElem::Func(Rc::new(ty_func)));
}

fn source_signature(sign_node: &SignatureNode, resolver: &mut Resolver) -> Signature {
    let in_ty = sign_node
        .in_type()
        .map(|in_node| get_ty_of_node(&in_node, resolver));
    let ret_ty = sign_node
        .ret_type()
        .map(|ret_node| get_ty_of_node(&ret_node, resolver));
    let args: Vec<ArgSignature> = sign_node
        .args()
        .map(|arg_node| -> ArgSignature {
            let arg_name = arg_node.name();
            let ty = arg_node
                .type_()
                .map(|ty_node| get_ty_of_node(&ty_node, resolver));
            ArgSignature::new(arg_name, ty, false)
        })
        .collect();
    let flags = sign_node
        .flags()
        .map(|flag_node| -> FlagSignature {
            let long_name = flag_node.long_name();
            let short_name = flag_node.short_name();
            let ty = flag_node
                .type_()
                .map(|ty_node| get_ty_of_node(&ty_node, resolver));
            FlagSignature::new(long_name, short_name, ty)
        })
        .collect();
    let var_arg = sign_node.var_arg().map(|var_arg_node| {
        let name = var_arg_node.name();
        let ty = var_arg_node
            .type_()
            .map(|ty_node| get_ty_of_node(&ty_node, resolver));
        VarArgSignature::new(name, ty)
    });
    Signature::new(args, var_arg, flags, in_ty, ret_ty)
}

fn get_ty_of_node(ty_node: &LuTypeNode, resolver: &mut Resolver) -> ValueType {
    let ty = ValueType::from_node(&ty_node.into_type(), &resolver.custom_types);
    resolver.ok_or_record_err(ty)
}
