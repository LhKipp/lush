use lu_error::LuResult;
use lu_syntax::ast::CmdStmtNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for CmdStmtNode {
    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        let cmd_storage = state.cmds.clone();
        let mut args = Vec::new();

        let result = if let Some((cmd_parts_count, cmd)) = cmd_storage
            .lock()
            .find_cmd_with_longest_match(&possibl_longest_name)
        {
            // Push cmd name as $args.0
            args.push(Value::String(
                possibl_longest_name[0..cmd_parts_count].join(" "),
            ));
            // Push all other arguments
            for arg in self.args().skip(cmd_parts_count) {
                // TODO remove partial result from eval call
                args.push(arg.evaluate(state)?);
            }

            state
                .scope
                .lock()
                .cur_mut_frame()
                .insert_var("args".to_string(), Value::Array(args));

            cmd.run(state)
        } else {
            // Cmd not found. Shell out
            todo!("Todo")
        };

        result
    }
}
