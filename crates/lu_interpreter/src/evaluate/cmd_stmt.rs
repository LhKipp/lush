use crate::{eval_function, evaluate::eval_prelude::*};
use crate::{Command, RunExternalCmd};
use lu_dbg::DbgIntervention;
use lu_syntax::ast::{CmdArgElement, CmdStmtNode};
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
        if is_dbg_session(&scope.lock())
            && (cmd.find_attr(CmdAttributeVariant::Impure).is_some()
                || cmd.find_attr(CmdAttributeVariant::PurityUnknown).is_some())
        {
            match lu_dbg::warn_unpure_cmd_call(&cmd, self.ast_id(), scope)? {
                Some(DbgIntervention::ContinueAsIfStmtRet(val)) => return Ok(val),
                None => {} // Okay nothing to do
            }
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
    // Insert flags
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
                let flag_name = long_flag.flag_name();
                match flag_usage_to_flag_arg(cmd_flags, |flag_sign| {
                    if flag_sign.long_name.as_ref() == Some(&flag_name) {
                        Some((flag_name.clone(), flag_sign.ty.clone()))
                    } else {
                        None
                    }
                }) {
                    Ok((flag_name, val)) => flag_vals.push((flag_name, val, long_flag.to_item())),
                    Err(flag_name) => last_seen_flag = Some((flag_name, long_flag.to_item())),
                }
            }
            CmdArgElement::ShortFlag(short_flag) => {
                let flag_name = short_flag.flag_name();
                match flag_usage_to_flag_arg(cmd_flags, |flag_sign| {
                    if flag_sign.short_name == Some(flag_name) {
                        Some((flag_name.to_string(), flag_sign.ty.clone()))
                    } else {
                        None
                    }
                }) {
                    Ok((flag_name, val)) => flag_vals.push((flag_name, val, short_flag.to_item())),
                    Err(flag_name) => last_seen_flag = Some((flag_name, short_flag.to_item())),
                }
            }
            CmdArgElement::ValueExpr(n) => {
                let val = n.evaluate(scope)?;
                if let Some((flag_name, usage_item)) = last_seen_flag.take() {
                    flag_vals.push((flag_name, val, usage_item));
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

/// Ok if flag is simple toggle and can be converted to a flag_arg, otherwise Err with flag_name
fn flag_usage_to_flag_arg<FindMapFn>(
    cmd_flags: &[FlagSignature],
    flag_sign_matches_passed_flag: FindMapFn,
) -> Result<(String, Value), String>
where
    FindMapFn: FnMut(&FlagSignature) -> Option<(String, ValueType)>,
{
    let (flag_name, flag_ty) = cmd_flags
        .iter()
        .find_map(flag_sign_matches_passed_flag)
        .expect("Flag will always be found");
    if flag_ty == ValueType::Bool {
        Ok((flag_name, true.into()))
    } else {
        Err(flag_name)
    }
}
