use super::handle_dbg_intervention_before;
use crate::special_cmds::{self, SELECT_CMD_NAME};
use crate::{eval_function, evaluate::eval_prelude::*};
use crate::{Command, RunExternalCmd};
use lu_interpreter_structs::special_cmds::{MATH_FN_NAME, SELECT_DEF_STRCT_DECL_ARG_NAME};
use lu_syntax::ast::{CmdArgElement, CmdStmtNode, HasAstId, MathExprNode};
use std::rc::Rc;

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, eval_args: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let cmd_name = self.get_cmd_name();
        let passed_flags = FlagVariant::convert(self.get_passed_flags());
        let cmd: Rc<dyn Command> =
            if let Some(cmd) = scope.lock().find_func(&cmd_name, &passed_flags) {
                cmd.clone()
            } else {
                RunExternalCmd::new(self.clone(), cmd_name).rced()
            };

        let grouped_args = evaluate_and_group_args(self.args(), &cmd.signature(), scope)?;

        // FROM HERE ONLY EVALUATION OF CMD FOLLOWS
        // REASON: Otherwise the following dbg_stmt may return this func to early
        if cmd.find_attr(CmdAttributeVariant::Impure).is_some()
            || cmd.find_attr(CmdAttributeVariant::PurityUnknown).is_some()
        {
            let dbg_result = lu_dbg::warn_unpure_cmd_call(&cmd, self.get_ast_id(), scope)?;
            handle_dbg_intervention_before!(dbg_result, scope);
        }

        // We need to prepare everything for the command to run properly.

        // Select the right scope to run the cmd
        let prev_scope_frame = select_scope_to_run_cmd_in(&cmd, scope);

        let cmd_call_frame =
            ScopeFrameTag::CmdCallFrame(cmd.name().to_string(), cmd.signature().req_flags());
        // Add a new frame
        scope.lock().push_frame(cmd_call_frame.clone());

        insert_cmd_args_into_scope(cmd.signature(), eval_args, grouped_args, scope);

        if self.get_cmd_name() == SELECT_CMD_NAME {
            let mut l_scope = scope.lock();
            let gen_strct_name = special_cmds::select_def_strct_name(&self.to_item());
            let strct_decl = l_scope
                .find_var(&gen_strct_name)
                .unwrap()
                .val
                .as_strct_decl()
                .expect("Must be strct decl")
                .clone();
            l_scope.get_cur_frame_mut().insert_var(Variable::new(
                SELECT_DEF_STRCT_DECL_ARG_NAME.to_string(),
                Value::StrctDecl(strct_decl),
                self.to_item().into(),
            ));
        }

        // And now we can finally run the cmd
        // See Function::run
        let cmd_result = if let Some(func) = cmd.as_function() {
            eval_function(func, scope)
        } else {
            cmd.run_cmd(scope)
        };

        // Cleanup in reverse order

        // poping the frame will also remove the vars
        scope.lock().pop_frame(&cmd_call_frame);

        if let Some(prev_scope_frame) = prev_scope_frame {
            scope.lock().set_cur_frame_id(prev_scope_frame);
        }

        Evaluator::lu_result_to_eval_result(cmd_result)
    }

    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
}

fn insert_cmd_args_into_scope(
    cmd_sign: &Signature,
    args: &[EvalArg],
    grouped_args: GroupedArgs,
    scope: &mut SyScope,
) -> () {
    let GroupedArgs {
        arg_vals,
        flag_vals,
    } = grouped_args;
    // Insert $in if given (should normaly be the case if cmd_stmt.eval is called from
    // piped_cmds_stmt)
    if let Some((val, val_decl)) = args.iter().find_map(|arg| arg.as_cmd_in_val()) {
        scope
            .lock()
            .get_cur_frame_mut()
            .insert_var(Variable::new_in(val.clone(), val_decl.clone().into()));
    };

    let mut arg_iter = arg_vals.into_iter();
    for arg in &cmd_sign.args {
        let val = if arg.is_opt {
            Value::new_optional(arg.ty.clone(), arg_iter.next())
        } else {
            arg_iter
                .next()
                .expect("Always present if ty_checking works")
        };
        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            arg.name.clone(),
            val,
            arg.decl.clone(),
        ));
    }
    if let Some(var_arg) = &cmd_sign.var_arg {
        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            var_arg.name.clone(),
            Value::new_array(arg_iter.collect()),
            var_arg.decl.clone(),
        ));
    } else {
        assert!(
            arg_iter.next().is_none(),
            "TyChecking does not work. Should be nothing left"
        );
    }

    // Insert non passed flags
    for flag_sign in &cmd_sign.flags {
        // Skip already inserted flags
        if flag_vals
            .iter()
            .any(|(flag_name, _, _)| flag_sign.is_named_by(flag_name))
        {
            continue;
        }
        // Non passed switches (flags with ty bool) are inserted as false
        let val = if flag_sign.ty == ValueType::Bool {
            false.into()
        } else {
            Value::new_optional(flag_sign.ty.clone(), None)
        };

        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            flag_sign.best_name(),
            val,
            flag_sign.decl.clone().into(),
        ));
    }

    // Insert passed flags
    for (flag_name, val, usage_item) in flag_vals {
        scope
            .lock()
            .get_cur_frame_mut()
            .insert_var(Variable::new(flag_name, val, usage_item));
    }
}

/// Returns the current frame id, iff another frame had to be selected to run cmd
fn select_scope_to_run_cmd_in(cmd: &Rc<dyn Command>, scope: &mut SyScope) -> Option<ScopeFrameId> {
    if let Some(cmd_parent_mod) = cmd.parent_module() {
        debug!(
            "Cmd is in different module. Selecting module: {}",
            cmd_parent_mod
        );
        let prev_id = scope.lock().get_cur_frame_id();
        scope
            .lock()
            .select_sf_frame(cmd_parent_mod)
            .expect(&format!(
                "Found cmd {} which has parent mod {}, which isn't in scope!",
                cmd.name(),
                cmd_parent_mod,
            ));
        Some(prev_id)
    } else {
        None
    }
}

struct GroupedArgs {
    arg_vals: Vec<Value>,
    flag_vals: Vec<(String, Value, SourceCodeItem)>,
}

macro_rules! insert_if_bool_flag_or_set_as_last_seen {
    ($flag_vals:ident, $last_seen_flag:ident, $cmd_flags:ident, $passed_flag:ident, $find_flag_sign_cls:expr) => {{
        let flag_sign = $cmd_flags
            .iter()
            .find($find_flag_sign_cls)
            .expect("Flag will always be found");
        if flag_sign.ty == ValueType::Bool {
            $flag_vals.push((flag_sign.best_name(), true.into(), $passed_flag.to_item()));
        } else {
            $last_seen_flag = Some((flag_sign, $passed_flag.to_item()));
        }
    }};
}

fn evaluate_and_group_args(
    args: impl Iterator<Item = CmdArgElement>,
    cmd_signature: &Signature,
    scope: &mut SyScope,
) -> Result<GroupedArgs, RetValOrErr> {
    let cmd_flags = &cmd_signature.flags;
    let mut arg_vals = vec![];
    // Flag name with value
    let mut flag_vals: Vec<(String, Value, SourceCodeItem)> = vec![];
    let mut last_seen_flag = None;
    debug!("Evaluating all cmd args");

    for (i, arg) in args.enumerate() {
        match arg {
            CmdArgElement::LongFlag(long_flag) => {
                insert_if_bool_flag_or_set_as_last_seen!(
                    flag_vals,
                    last_seen_flag,
                    cmd_flags,
                    long_flag,
                    |flag_sign| flag_sign.long_name.as_ref() == Some(&long_flag.flag_name())
                );
            }
            CmdArgElement::ShortFlag(short_flag) => {
                insert_if_bool_flag_or_set_as_last_seen!(
                    flag_vals,
                    last_seen_flag,
                    cmd_flags,
                    short_flag,
                    |flag_sign| flag_sign.short_name.as_ref() == Some(&short_flag.flag_name())
                );
            }
            CmdArgElement::ValueExpr(n) => {
                let expected_val_ty = last_seen_flag
                    .as_ref()
                    .map(|(flag_sign, _)| &flag_sign.ty)
                    .or_else(|| {
                        cmd_signature
                            .args
                            .get(i)
                            .map(|arg| &arg.ty)
                            .or(cmd_signature.var_arg.as_ref().map(|var_arg| &var_arg.ty))
                    });
                let val = match (n.as_math_expr(), expected_val_ty) {
                    (Some(math_node), Some(ValueType::Func(expected_sign))) => {
                        math_expr_to_cmd_value(math_node, expected_sign, scope)
                    }
                    _ => n.evaluate(scope)?,
                };
                if let Some((flag_sign, passed_flag_decl)) = last_seen_flag.take() {
                    let val = if flag_sign.is_opt {
                        Value::Optional {
                            inner_ty: flag_sign.ty.clone(),
                            val: Some(Box::new(val)),
                        }
                    } else {
                        val
                    };

                    flag_vals.push((
                        flag_sign.best_name(),
                        val,
                        // TODO maybe include also val? Needs merging of SourceCodeItem
                        passed_flag_decl,
                    ));
                } else {
                    arg_vals.push(val);
                }
            }
        }
    }
    debug!("Found {} cmd arguments", arg_vals.len());
    Ok(GroupedArgs {
        arg_vals,
        flag_vals,
    })
}

fn math_expr_to_cmd_value(
    math_node: &MathExprNode,
    expected_sign: &Signature,
    scope: &mut SyScope,
) -> Value {
    // a math_expr as fn runs in the source_file where it has been declared
    let mod_path = scope
        .lock()
        .get_cur_mod_frame()
        .expect("Math expr always inside lufile")
        .get_mod_tag()
        .id
        .clone();
    let math_as_func = Function::new(
        MATH_FN_NAME.into(),
        expected_sign.clone(),
        vec![],
        math_node.clone().into(),
        mod_path,
    );
    Value::new_func(Rc::new(math_as_func))
}
