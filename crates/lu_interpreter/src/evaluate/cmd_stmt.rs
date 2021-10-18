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
        let passed_flags = self
            .get_passed_flags()
            .map(|elem| FlagVariant::from_node(&elem))
            .collect::<Vec<_>>();
        let cmd: Rc<dyn Command> =
            if let Some(cmd) = scope.lock().find_func(&cmd_name, &passed_flags) {
                cmd.clone()
            } else {
                RunExternalCmd::new(self.clone(), cmd_name).rced()
            };

        debug!("Evaluating all cmd args");
        // let cmd_sign = cmd.signature();

        let mut arg_vals = vec![];
        // let mut flags = HashMap::new();
        // let mut last_seen_flag = None;
        for arg in self.args() {
            match arg {
                CmdArgElement::LongFlag(_) => {
                    todo!();
                    // if let Some(last_flag) = last_seen_flag {
                    // }
                    // last_seen_flag = Some(FlagElement::LongFlag(f));
                }
                CmdArgElement::ShortFlag(_) => {
                    todo!();
                }
                CmdArgElement::ValueExpr(n) => {
                    arg_vals.push(n.evaluate(scope)?);
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
