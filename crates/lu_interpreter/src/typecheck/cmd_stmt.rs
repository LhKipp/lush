#![allow(unused_imports)]
#![allow(unused_variables)]
use std::iter;

use log::debug;
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::ErrorContainer;
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
        let ret_ty = if let Some((name_args_split_i, var)) = ty_state
            .scope
            .find_var_with_longest_match(&possibl_longest_name)
        {
            let args = self.name_with_args().skip(name_args_split_i);
            let var = var.clone();

            if let Some(func) = ty_state.tc_func_table.get(&var).cloned() {
                assert!(matches!(var.decl.as_ref().unwrap(), VarDeclNode::FnStmt(_)));
                ty_check_cmd_args(
                    self,
                    args,
                    &func,
                    var.decl.as_ref().unwrap().into_item(),
                    ty_state,
                );
                func.ret_ty.clone()
            } else {
                // We have found such a var, but its not a function
                // This error should be catched more elaborated in special check for this
                debug!(
                    "Expected {} to be a function, but isn't present in tc_func_table",
                    var.name
                );

                ty_state.new_term_key_concretiziesd(self.into_item(), ValueType::Error)
            }
        } else {
            // External cmds return string
            ty_state.new_term_key_concretiziesd(self.into_item(), ValueType::String)
        };

        Some(ty_state.new_term_key_equated(self.into_item(), ret_ty))
    }
}

fn ty_check_cmd_args<ArgIter: Iterator<Item = ValueExprElement>>(
    cmd_node: &CmdStmtNode,
    args: ArgIter,
    func: &TcFunc,
    fn_decl: SourceCodeItem,
    ty_state: &mut TypeChecker,
) {
    let mut func_arg_ty_iter = func.args_ty.iter();

    for arg in args {
        match func_arg_ty_iter.next() {
            Some(k) => {
                ty_state.new_term_key_equated(arg.into_item(), *k);
            }
            None => {
                if let Some(var_arg_ty) = func.var_arg_ty {
                    ty_state.new_term_key_equated(arg.into_item(), var_arg_ty);
                } else {
                    // Found unexpected argument
                    ty_state.errors.push(
                        TyErr::UnexpectedArg {
                            arg: arg.into_item(),
                            fn_decl: fn_decl.clone(),
                        }
                        .into(),
                    )
                }
            }
        }
    }

    for non_passed_arg in func_arg_ty_iter {
        let arg_decl = ty_state.get_item_of(non_passed_arg).clone();
        ty_state.errors.push(
            TyErr::UnsatisfiedArg {
                arg_decl,
                cmd_stmt: cmd_node.into_item(),
            }
            .into(),
        )
    }
}
