use log::debug;
use lu_syntax::ast::CmdStmtNode;
use lu_value::Value;

use crate::{
    callable::Callable, Command, EvalArg, Evaluable, Evaluator, RetValOrErr, RunExternalCmd,
    Variable, ARGS_VAR_NAME,
};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> Result<Value, RetValOrErr> {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        let (cmd_parts_count, cmd): (usize, Callable) = if let Some((cmd_parts_count, callable)) =
            state
                .scope
                .lock()
                .find_cmd_with_longest_match(&possibl_longest_name)
        {
            (cmd_parts_count, callable.clone())
        } else {
            (
                1,
                Callable::ExternalCmd(RunExternalCmd::new(
                    self.clone(),
                    possibl_longest_name[0].clone(),
                )),
            )
        };

        let cmd_name = possibl_longest_name[0..cmd_parts_count].join(" ");
        debug!("Calling cmd: {}", cmd_name);

        // TODO map values to signature names
        // Push real cmd arguments (excluding cmd name, as $args)
        let mut args = Vec::new();
        for arg in self.name_with_args().skip(cmd_parts_count) {
            // TODO remove partial result from eval call
            args.push(arg.evaluate(state)?);
        }

        state.scope.lock().cur_mut_frame().insert(
            ARGS_VAR_NAME.to_string(),
            // TODO correct source
            Variable::new_args(Value::new_array(args)),
        );

        Evaluator::lu_result_to_eval_result(cmd.run(state))
    }
}
