use super::handle_dbg_intervention_before;
use crate::{eval_function, evaluate::eval_prelude::*};
use crate::{Command, RunExternalCmd};
use lu_syntax::ast::{CmdArgElement, CmdStmtNode, HasAstId};
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

        let grouped_args = evaluate_and_group_args(self.args(), &cmd.signature().flags, scope)?;

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
        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            arg.name.clone(),
            arg_iter
                .next()
                .expect("Always present if ty_checking works"),
            VarDeclNode::CatchAll(arg.decl.clone()),
        ));
    }
    if let Some(var_arg) = &cmd_sign.var_arg {
        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            var_arg.name.clone(),
            Value::new_array(arg_iter.collect()),
            VarDeclNode::CatchAll(var_arg.decl.clone()),
        ));
    } else {
        assert!(
            arg_iter.next().is_none(),
            "TyChecking does not work. Should be nothing left"
        );
    }

    // Insert non passed bool flags (flag switches) as false if they are not present
    for flag_sign in &cmd_sign.flags {
        if flag_sign.ty == ValueType::Bool
            // And not yet passed
            && !flag_vals.iter().any(|(flag_name, _, _)| {
                flag_sign.is_named_by(flag_name)
            })
        {
            scope.lock().get_cur_frame_mut().insert_var(Variable::new(
                flag_sign.best_name(),
                Value::Bool(false),
                flag_sign.decl.clone().into(),
            ));
        }
    }

    // Insert passed flags
    for (flag_name, val, usage_item) in flag_vals {
        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            flag_name,
            val,
            VarDeclNode::CatchAll(usage_item),
        ));
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
    cmd_flags: &[FlagSignature],
    scope: &mut SyScope,
) -> Result<GroupedArgs, RetValOrErr> {
    let mut arg_vals = vec![];
    // Flag name with value
    let mut flag_vals: Vec<(String, Value, SourceCodeItem)> = vec![];
    let mut last_seen_flag = None;
    debug!("Evaluating all cmd args");

    for arg in args {
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
                let val = n.evaluate(scope)?;
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
