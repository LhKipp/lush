use std::rc::Rc;

use crate::{eval_function, evaluate::eval_prelude::*};
use lu_interpreter_structs::Value;
use lu_syntax::ast::{CmdArgElement, CmdStmtNode};

use crate::{Command, EvalArg, EvalResult, Evaluable, Evaluator, RunExternalCmd, Variable};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
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

        let cmd_flags = &cmd.signature().flags;
        let mut arg_vals = vec![];
        // Flag name with value
        let mut flag_vals: Vec<(String, Value)> = vec![];
        let mut last_seen_flag = None;
        debug!("Evaluating all cmd args");

        for arg in self.args() {
            match arg {
                CmdArgElement::LongFlag(long_flag) => {
                    let flag_name = long_flag.flag_name();
                    match flag_usage_to_flag_arg(&cmd_flags, |flag_sign| {
                        if flag_sign.long_name.as_ref() == Some(&flag_name) {
                            Some((flag_name.clone(), flag_sign.ty.clone()))
                        } else {
                            None
                        }
                    }) {
                        Ok(flag_arg) => flag_vals.push(flag_arg),
                        Err(flag_name) => last_seen_flag = Some(flag_name),
                    }
                }
                CmdArgElement::ShortFlag(short_flag) => {
                    let flag_name = short_flag.flag_name();
                    match flag_usage_to_flag_arg(&cmd_flags, |flag_sign| {
                        if flag_sign.short_name == Some(flag_name) {
                            Some((flag_name.to_string(), flag_sign.ty.clone()))
                        } else {
                            None
                        }
                    }) {
                        Ok(flag_arg) => flag_vals.push(flag_arg),
                        Err(flag_name) => last_seen_flag = Some(flag_name),
                    }
                }
                CmdArgElement::ValueExpr(n) => {
                    let val = n.evaluate(scope)?;
                    if let Some(last_seen_flag) = last_seen_flag.take() {
                        flag_vals.push((last_seen_flag, val));
                    } else {
                        arg_vals.push(val);
                    }
                }
            }
        }
        debug!("Found {} cmd arguments", arg_vals.len());

        // We need to prepare everything for the command to run properly.

        // Select the right scope to run the cmd
        let prev_scope_frame = if let Some(cmd_parent_mod) = cmd.parent_module() {
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
        };

        let cmd_call_frame =
            ScopeFrameTag::CmdCallFrame(cmd.name().to_string(), cmd.signature().req_flags());
        // Add a new frame
        scope.lock().push_frame(cmd_call_frame.clone());
        // 2. Insert cmd_args as variables

        // Insert $in if given (should normaly be the case if cmd_stmt.eval is called from
        // piped_cmds_stmt)
        if let Some((val, val_decl)) = args.iter().find_map(|arg| arg.as_cmd_in_val()) {
            scope
                .lock()
                .get_cur_frame_mut()
                .insert_var(Variable::new_in(val.clone(), val_decl.clone().into()));
        };

        let sign = cmd.signature();
        let mut arg_iter = arg_vals.into_iter();
        for arg in &sign.args {
            scope.lock().get_cur_frame_mut().insert_var(Variable::new(
                arg.name.clone(),
                arg_iter
                    .next()
                    .expect("Always present if ty_checking works"),
                VarDeclNode::CatchAll(arg.decl.clone()),
            ));
        }
        if let Some(var_arg) = &sign.var_arg {
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

        // And now we can finally run the cmd
        // See Function::run
        let cmd_result = if let Some(func) = cmd.as_function() {
            eval_function(func, scope)
        } else {
            cmd.run_cmd(scope)
        };

        // Cleanup in reverse order
        scope.lock().pop_frame(&cmd_call_frame);

        if let Some(prev_scope_frame) = prev_scope_frame {
            scope.lock().set_cur_frame_id(prev_scope_frame);
        }

        Evaluator::lu_result_to_eval_result(cmd_result)
    }
}

/// Ok if flag is simple toggle and can be converted to a flag_arg, otherwise Err with flag_name
fn flag_usage_to_flag_arg<FindMapFn>(
    cmd_flags: &Vec<FlagSignature>,
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
