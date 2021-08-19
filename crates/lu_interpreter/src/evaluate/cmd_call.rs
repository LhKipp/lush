use log::debug;
use lu_error::LuResult;
use lu_syntax::ast::CmdStmtNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for CmdStmtNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        let cmd_storage = state.cmds.clone();
        let mut args = Vec::new();

        let result = if let Some((cmd_parts_count, cmd)) = cmd_storage
            .lock()
            .find_cmd_with_longest_match(&possibl_longest_name)
        {
            let cmd_name = possibl_longest_name[0..cmd_parts_count].join(" ");
            debug!("Calling cmd: {}", cmd_name);

            // Push cmd name as $args.0
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
                .insert_var("args".to_string(), Value::new_array(args));

            cmd.run(state)
        } else {
            // Cmd not found. Shell out
            todo!("Todo")
        };

        result
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_syntax::ast::CmdStmtNode;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/cmd_call/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<CmdStmtNode>(SourceCode::Text(s.to_string()))
    }
}
