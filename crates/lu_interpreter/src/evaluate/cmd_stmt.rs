use std::rc::Rc;

use crate::{eval_function, evaluate::eval_prelude::*};
use lu_interpreter_structs::Value;
use lu_syntax::ast::CmdStmtNode;

use crate::{Command, EvalArg, EvalResult, Evaluable, Evaluator, RunExternalCmd, Variable};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let cmd_name = self.get_cmd_name();
        let cmd: Rc<dyn Command> = if let Some(cmd) = scope.lock().find_func(&cmd_name) {
            cmd.clone()
        } else {
            RunExternalCmd::new(self.clone(), cmd_name).rced()
        };

        debug!("Evaluating all cmd args");
        let mut arg_vals = vec![];
        for arg in self.args() {
            arg_vals.push(arg.evaluate(scope)?);
        }
        debug!("Found {} cmd arguments", arg_vals.len());

        // We need to prepare everything for the command to run properly.
        // 1. Add a new frame
        scope
            .lock()
            .push_frame(ScopeFrameTag::CmdCallFrame(cmd.name().to_string()));
        // 2. Insert cmd_args as variables
        let sign = cmd.signature();
        let mut arg_iter = arg_vals.into_iter();
        debug!("{:?}", sign);
        for arg in &sign.args {
            debug!("{:?}", arg);
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

        // See Function::run
        let cmd_result = if let Some(func) = cmd.as_function() {
            eval_function(func, scope)
        } else {
            cmd.run_cmd(scope)
        };

        scope
            .lock()
            .pop_frame(&ScopeFrameTag::CmdCallFrame(cmd.name().to_string()));

        Evaluator::lu_result_to_eval_result(cmd_result)
    }
}
