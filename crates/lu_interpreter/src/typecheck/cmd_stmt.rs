#![allow(unused_imports)]
#![allow(unused_variables)]
use log::{debug, warn};
use lu_error::{lu_source_code_item, SourceCodeItem, TyErr};
use lu_interpreter_structs::{
    external_cmd,
    special_cmds::{MATH_FN_NAME, SELECT_CMD_NAME},
    FlagSignature, FlagVariant, ScopeFrameTag, Value,
};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdArgElement, CmdStmtNode, LetStmtNode, MathExprNode, ValueExprElement},
    AstElement, AstNode, AstToken,
};
use rusttyc::TcKey;
use std::{collections::HashMap, iter};

use crate::typecheck::cmd_select::do_extra_ty_check_select_cmd;
use crate::{TcFunc, TyCheckState, TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable};

impl TypeCheck for CmdStmtNode {
    fn do_typecheck(
        &self,
        args: &[TypeCheckArg],
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
                // TODO use get_tc_cmd_from_cmd_usage
                .get_tc_cmd_from_rc_cmd(&cmd)
                .expect("If cmd is found in scope it must be found in ty_state")
        } else {
            TcFunc::from_signature(&external_cmd::external_cmd_signature(), ty_state)
        };

        if let Some(in_key) = args.iter().find_map(|arg| arg.as_cmd_stmt()) {
            ty_state.equate_keys(cmd_keys.in_key.clone(), *in_key);
        } else {
            warn!("Cmd stmt arg should always be passed");
            ty_state.concretizes_key(cmd_keys.in_key.clone(), ValueType::Nil);
        }

        // Ty check args
        ty_check_cmd_args_and_flags(self, self.args(), &cmd_keys, ty_state);

        if self.get_cmd_name() == SELECT_CMD_NAME {
            if let Some(key) = do_extra_ty_check_select_cmd(self, args, ty_state) {
                return Some(key);
            }
        }
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
                    warn!("Not promoting math expr to function");
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
        ty_check_math_expr_as_fn(passed_math_expr, expected_fn_ty, ty_state);
    } else {
        let passed_arg_key = passed_arg
            .typecheck(ty_state)
            .expect("Arg always returns a key");
        ty_state.equate_keys(passed_arg_key, *called_func_arg_tc);
    };
}

fn ty_check_math_expr_as_fn(
    passed_math_expr: &MathExprNode,
    expected_fn_ty: TcFunc,
    ty_state: &mut TyCheckState,
) {
    // TODO assert expected_fn_ty is simple
    let fn_frame = ScopeFrameTag::TyCFnFrame(MATH_FN_NAME.into(), vec![]);
    let (_, frame) = ty_state.scope.push_frame(fn_frame.clone());
    // Insert vars
    for (arg, key) in &expected_fn_ty.args_keys {
        let arg_key = ty_state.insert_var(arg.to_var());
        ty_state.equate_keys(arg_key, key.clone());
    }
    let in_key = ty_state.insert_var(Variable::new_in(Value::Nil, lu_source_code_item!().into()));
    ty_state.equate_keys(in_key, expected_fn_ty.in_key.clone());

    if let Some(math_expr_ret) = passed_math_expr.typecheck(ty_state) {
        ty_state.equate_keys(math_expr_ret, expected_fn_ty.ret_key.clone());
    } else {
        ty_state.concretizes_key(expected_fn_ty.ret_key.clone(), ValueType::Nil);
    }

    ty_state.scope.pop_frame(&fn_frame);
}
