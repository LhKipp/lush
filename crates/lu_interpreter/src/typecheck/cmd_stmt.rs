#![allow(unused_imports)]
#![allow(unused_variables)]
use std::iter;

use log::{debug, warn};
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdStmtNode, LetStmtNode, ValueExprElement},
    AstElement, AstNode,
};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{
    Callable, TcFunc, TyCheckState, TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable,
};

impl TypeCheck for CmdStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        debug!("Scope: {:?}", ty_state.scope);
        debug!("Cur Scope Frame: {:?}", ty_state.scope.cur_frame());
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        // Finding result type here
        let ret_ty = if let Some((name_args_split_i, called_func)) =
            ty_state.expect_callable_with_longest_name(&possibl_longest_name, self)
        {
            let args = self.name_with_args().skip(name_args_split_i);
            ty_check_cmd_args(self, args, &called_func, ty_state);
            called_func.ret_key.clone()
        } else {
            // TODO this is wrong
            // External cmds return string
            ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::String)
        };

        Some(ty_state.new_term_key_equated(self.to_item(), ret_ty))
    }
}

fn ty_check_cmd_args<ArgIter: Iterator<Item = ValueExprElement>>(
    cmd_node: &CmdStmtNode,
    args: ArgIter,
    called_func: &TcFunc,
    ty_state: &mut TyCheckState,
) {
    let mut called_func_arg_tc_iter = called_func.args_keys.iter();

    for arg in args {
        match called_func_arg_tc_iter.next() {
            Some(called_func_arg_tc) => {
                ty_check_cmd_arg(arg, called_func_arg_tc, cmd_node, ty_state);
            }
            None => {
                if let Some(var_arg_ty) = called_func.var_arg_key {
                    ty_state.new_term_key_equated(arg.to_item(), var_arg_ty);
                } else {
                    // Found unexpected argument
                    let called_func_decl = ty_state.get_item_of(&called_func.self_key).clone();
                    ty_state.push_err(
                        TyErr::UnexpectedArg {
                            arg: arg.to_item(),
                            fn_decl: called_func_decl.clone(),
                        }
                        .into(),
                    )
                }
            }
        }
    }

    for non_passed_arg in called_func_arg_tc_iter {
        let arg_decl = ty_state.get_item_of(non_passed_arg).clone();
        ty_state.push_err(
            TyErr::UnsatisfiedArg {
                arg_decl,
                cmd_stmt: cmd_node.to_item(),
            }
            .into(),
        )
    }
}

fn ty_check_cmd_arg(
    passed_arg: ValueExprElement,
    called_func_arg_tc: &TcKey,
    cmd_node: &CmdStmtNode,
    ty_state: &mut TyCheckState,
) {
    // Check whether the expected arg is a function
    if let Some(expected_fn_ty) = ty_state.get_tc_func(called_func_arg_tc).cloned() {
        debug!(
            "TyChecking passed_arg: {:?}, against expected_fn_ty",
            passed_arg.text()
        );
        // The function expects a function as an arg
        // We need to ty check the passed arg against the accepted func
        match passed_arg {
            ValueExprElement::MathExpr(_) => {
                todo!("Expected func, provided math expr. This should work. Hack around here ")
            }
            _ => {
                let passed_arg_key = passed_arg.typecheck(ty_state).expect("Arg always has key");
                if let Some(passed_fn_ty) = ty_state.expect_callable_from_key(passed_arg_key) {
                    expected_fn_ty.equate_with(&passed_fn_ty, ty_state);
                    true
                } else {
                    false
                }
            }
        };
    } else {
        warn!("Array as cmd arg not handled special");
        let passed_arg_key = passed_arg
            .typecheck(ty_state)
            .expect("Arg always returns a key");
        // Everything else is a primitive type. No deeper ty check necessary
        ty_state.equate_keys(passed_arg_key, *called_func_arg_tc);
    }
}
