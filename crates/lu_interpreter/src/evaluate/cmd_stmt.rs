use log::debug;
use lu_error::LuResult;
use lu_syntax::ast::CmdStmtNode;
use lu_value::Value;

use crate::{
    command::RunExternalCmd, function::Callable, Command, EvalArg, Evaluable, Interpreter, Variable,
};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
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
                RunExternalCmd {
                    cmd_node: self.clone(),
                }
                .into(),
            )
        };

        let cmd_name = possibl_longest_name[0..cmd_parts_count].join(" ");
        debug!("Calling cmd: {}", cmd_name);

        // Push cmd name as $args.0
        let mut args = Vec::new();
        args.push(Value::String(cmd_name));
        // Push all other arguments
        for arg in self.args().skip(cmd_parts_count) {
            // TODO remove partial result from eval call
            args.push(arg.evaluate(state)?);
        }

        state
            .scope
            .lock()
            .cur_mut_frame()
            .insert_var(Variable::new("args".to_string(), Value::new_array(args)));

        cmd.run(state)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_syntax::ast::SourceFileNode;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/cmd_stmt/general.json_test")]
    fn general_cmd_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/cmd_stmt/external.json_test")]
    fn external_cmd_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
