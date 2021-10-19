#![allow(unused_imports)]
#![allow(unused_variables)]
use std::iter;

use log::{debug, warn};
use lu_error::{SourceCodeItem, TyErr};
use lu_interpreter_structs::{FlagVariant, Value};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdArgElement, CmdStmtNode, LetStmtNode, ValueExprElement},
    AstElement, AstNode,
};
use rusttyc::TcKey;

use crate::{TcFunc, TyCheckState, TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable};

impl TypeCheck for CmdStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        debug!("Cur Scope Frame: {}", ty_state.scope.get_cur_frame());

        // Finding result type here
        let passed_flags = FlagVariant::convert(self.get_passed_flags());
        let called_cmd = ty_state
            .scope
            .find_func(&self.get_cmd_name(), &passed_flags)
            .cloned();

        let ret_ty = if let Some(cmd) = called_cmd {
            let cmd_keys = ty_state
                .get_tc_cmd_from_rc_cmd(&cmd)
                .expect("If cmd is found in scope it must be found in ty_state");
            let args = self.args();
            ty_check_cmd_args(self, args, &cmd_keys, ty_state);
            ty_state.new_term_key_equated(self.to_item(), cmd_keys.ret_key.clone())
        } else {
            ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::String)
        };

        Some(ty_state.new_term_key_equated(self.to_item(), ret_ty))
    }
}

fn ty_check_cmd_args<ArgIter: Iterator<Item = CmdArgElement>>(
    cmd_node: &CmdStmtNode,
    args: ArgIter,
    called_func: &TcFunc,
    ty_state: &mut TyCheckState,
) {
    let mut called_func_arg_tc_iter = called_func.args_keys.iter();

    for arg in args {
        let arg = if let CmdArgElement::ValueExpr(n) = arg {
            n
        } else {
            unreachable!("TODO ty check cmd flags")
        };
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
    debug!(
        "TyChecking passed_arg: {}, against {}",
        passed_arg.text(),
        ty_state.get_item_of(called_func_arg_tc).content
    );

    // Check whether we have to fixup the MathExpr to become a function
    let passed_arg_key = if let (Some(passed_math_expr), Some(expected_fn_ty)) = (
        passed_arg.as_math_expr(),
        ty_state.get_tc_func(called_func_arg_tc).cloned(),
    ) {
        todo!("Expected func, provided math expr. This should work. Hack around here ")
        // We need to make the math expr to a func
    } else {
        passed_arg
            .typecheck(ty_state)
            .expect("Arg always returns a key")
    };

    ty_state.equate_keys(passed_arg_key, *called_func_arg_tc);
}
