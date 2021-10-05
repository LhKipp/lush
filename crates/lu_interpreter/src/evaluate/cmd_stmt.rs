use std::rc::Rc;

use crate::evaluate::eval_prelude::*;
use log::debug;
use lu_interpreter_structs::Value;
use lu_syntax::ast::CmdStmtNode;

use crate::{Command, EvalArg, EvalResult, Evaluable, Evaluator, RunExternalCmd, Variable};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        let (cmd_parts_count, cmd): (usize, Rc<dyn Command>) =
            if let Some((cmd_parts_count, callable)) = scope
                .lock()
                .find_cmd_with_longest_match(&possibl_longest_name)
            {
                (cmd_parts_count, callable.clone())
            } else {
                (
                    1,
                    RunExternalCmd::new(self.clone(), possibl_longest_name[0].clone()).rced(),
                )
            };

        let cmd_name = possibl_longest_name[0..cmd_parts_count].join(" ");
        debug!("Calling cmd: {}", cmd_name);

        // TODO map values to signature names
        // Push real cmd arguments (excluding cmd name, as $args)
        let mut args = Vec::new();
        for arg in self.name_with_args().skip(cmd_parts_count) {
            // TODO remove partial result from eval call
            args.push(arg.evaluate(scope)?);
        }

        scope.lock().cur_mut_frame().insert(
            "args".to_string(),
            // TODO correct source
            Variable::new_args(Value::new_array(args)),
        );

        Evaluator::lu_result_to_eval_result(cmd.run_cmd(scope))
    }
}
