#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{collections::HashMap, iter};

use log::{debug, warn};
use lu_error::{SourceCodeItem, TyErr};
use lu_interpreter_structs::{external_cmd, FlagSignature, FlagVariant, Value};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdArgElement, CmdStmtNode, LetStmtNode, ValueExprElement},
    AstElement, AstNode, AstToken,
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
        let cmd_keys = if let Some(cmd) = called_cmd {
            ty_state
                .get_tc_cmd_from_rc_cmd(&cmd)
                .expect("If cmd is found in scope it must be found in ty_state")
        } else {
            TcFunc::from_signature(&external_cmd::external_cmd_signature(), ty_state)
        };

        // Ty check args
        ty_check_cmd_args_and_flags(self, self.args(), &cmd_keys, ty_state);

        // TODO ty check redir stmt
        Some(ty_state.new_term_key_equated(self.to_item(), cmd_keys.ret_key))
    }
}

fn ty_check_cmd_args_and_flags<ArgIter: Iterator<Item = CmdArgElement>>(
    cmd_node: &CmdStmtNode,
    mut args: ArgIter,
    called_func: &TcFunc,
    ty_state: &mut TyCheckState,
) {
    let mut called_func_arg_tc_iter = called_func.args_keys.iter();
    // Flags that are required but not passed
    let mut missing_called_func_req_flags: Vec<_> = called_func
        .flags_keys
        .iter()
        .filter(|(flag, _)| flag.is_required())
        .map(|(flag, _)| flag.clone())
        .collect();

    while let Some(next_arg) = args.next() {
        match next_arg {
            CmdArgElement::ShortFlag(n) => ty_check_flag(
                &mut args,
                |flag_sign| flag_sign.short_name == Some(n.flag_name()),
                called_func,
                n.to_item(),
                &mut missing_called_func_req_flags,
                ty_state,
            ),
            CmdArgElement::LongFlag(n) => ty_check_flag(
                &mut args,
                |flag_sign| flag_sign.long_name == Some(n.flag_name()),
                called_func,
                n.to_item(),
                &mut missing_called_func_req_flags,
                ty_state,
            ),
            CmdArgElement::ValueExpr(arg) => {
                match called_func_arg_tc_iter.next() {
                    Some((_, called_func_arg_tc)) => {
                        ty_check_cmd_arg(arg, called_func_arg_tc, cmd_node, ty_state);
                    }
                    None => {
                        if let Some(var_arg_ty) = called_func.var_arg_key {
                            ty_check_cmd_arg(arg, &var_arg_ty, cmd_node, ty_state);
                        } else {
                            // Found unexpected argument
                            let called_func_decl =
                                ty_state.get_item_of(&called_func.self_key).clone();
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
        }
    }

    for (arg, non_passed_arg) in called_func_arg_tc_iter {
        if arg.is_opt {
            // Okay. optional arg not passed.
            continue;
        }
        let arg_decl = ty_state.get_item_of(non_passed_arg).clone();
        ty_state.push_err(
            TyErr::UnsatisfiedArg {
                arg_decl,
                cmd_stmt: cmd_node.to_item(),
            }
            .into(),
        )
    }

    for non_passed_flag in missing_called_func_req_flags {
        ty_state.push_err(
            TyErr::NotPassedRequiredFlag {
                flag_decl: non_passed_flag.decl.clone(),
                cmd_stmt: cmd_node.to_item(),
            }
            .into(),
        );
    }
}

fn ty_check_flag<ArgIter: Iterator<Item = CmdArgElement>, P>(
    args: &mut ArgIter,
    mut flag_sign_matches_usage: P,
    called_func: &TcFunc,
    flag_usage: SourceCodeItem,
    missing_called_func_req_flags: &mut Vec<FlagSignature>,
    ty_state: &mut TyCheckState,
) where
    P: FnMut(&FlagSignature) -> bool,
{
    if let Some((flag, key)) = called_func.flags_keys.iter().find_map(|(flag, key)| {
        if flag_sign_matches_usage(flag) {
            Some((flag, key))
        } else {
            None
        }
    }) {
        // Found passed flag.
        if flag.is_required() {
            if let Some(flag_pos) = missing_called_func_req_flags
                .iter()
                .position(|missing_flag| missing_flag == flag)
            {
                missing_called_func_req_flags.remove(flag_pos);
            }
        }

        if flag.ty != ValueType::Bool {
            // next arg must be argument to flag
            match args.next() {
                Some(CmdArgElement::ValueExpr(arg_val)) => {
                    let arg_val_key = arg_val.typecheck(ty_state).unwrap();
                    ty_state.concretizes_key(arg_val_key, flag.ty.clone());
                }
                _ => {
                    ty_state.push_err(TyErr::FlagWithoutArgument(flag_usage).into());
                }
            }
        }
    } else {
        ty_state.push_err(TyErr::PassingOfNotDeclaredFlag(flag_usage).into());
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
