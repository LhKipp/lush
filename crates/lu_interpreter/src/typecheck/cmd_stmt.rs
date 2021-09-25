#![allow(unused_imports)]
#![allow(unused_variables)]
use std::iter;

use log::debug;
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdStmtNode, LetStmtNode, ValueExprElement},
    AstElement, AstNode,
};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{
    Callable, TcFunc, TypeCheck, TypeCheckArg, TypeChecker, ValueType, VarDeclNode, Variable,
};

impl TypeCheck for CmdStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TypeChecker,
    ) -> Option<TcKey> {
        debug!("Scope: {:?}", ty_state.scope);
        debug!("Cur Scope Frame: {:?}", ty_state.scope.cur_frame());
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        // Finding result type here
        let ret_ty = if let Some((name_args_split_i, called_func)) =
            ty_state.find_callable(&possibl_longest_name, self)
        {
            let args = self.name_with_args().skip(name_args_split_i);
            ty_check_cmd_args(self, args, &called_func, ty_state);
            called_func.ret_ty.clone()
        } else {
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
    ty_state: &mut TypeChecker,
) {
    let mut func_arg_ty_iter = called_func.args_ty.iter();

    for arg in args {
        match func_arg_ty_iter.next() {
            Some(func_arg_ty) => {
                ty_check_cmd_arg(arg, func_arg_ty, cmd_node, ty_state);
            }
            None => {
                if let Some(var_arg_ty) = called_func.var_arg_ty {
                    ty_state.new_term_key_equated(arg.to_item(), var_arg_ty);
                } else {
                    // Found unexpected argument
                    let called_func_decl = ty_state.get_expr_of(called_func.self_key).clone();
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

    for non_passed_arg in func_arg_ty_iter {
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
    expected_arg_ty: &TcKey,
    cmd_node: &CmdStmtNode,
    ty_state: &mut TypeChecker,
) {
    // Check whether the expected arg is a function
    if let Some(expected_fn_ty) = ty_state.get_tc_func(expected_arg_ty).cloned() {
        debug!(
            "TyChecking passed_arg: {:?}, against expected_fn_ty",
            passed_arg.text()
        );
        // The function expects a function as an arg
        // We need to ty check the passed arg against the accepted func
        let matched = match passed_arg {
            ValueExprElement::ValuePathExpr(ref passed_var_path) => {
                let var_repr = (
                    passed_var_path.var_name_parts()[0].clone(),
                    passed_var_path.to_item(),
                );
                let passed_var_key = ty_state.expect_key_of_var(var_repr);
                let passed_arg_key =
                    ty_state.new_term_key_equated(passed_arg.to_item(), passed_var_key);
                // Check that var_key is a func_key
                if let Some(passed_fn_ty) = ty_state.tc_func_table.get(&passed_var_key).cloned() {
                    expected_fn_ty.equate_with(&passed_fn_ty, ty_state);
                    true
                } else {
                    false
                }
            }
            ValueExprElement::MathExpr(_) => {
                todo!("Expected func, provided math expr. This should work")
            }
            _ => false,
        };

        if !matched {
            // Expected_ty did not match with passed arg. We must generate an error.
            // We call new_term_key_equated, as that goes then through a unified interface
            ty_state.new_term_key_equated(passed_arg.to_item(), *expected_arg_ty);
        }
    } else {
        // Everything else is a primitive type. No deeper ty check necessary
        ty_state.new_term_key_equated(passed_arg.to_item(), *expected_arg_ty);
    }
}
