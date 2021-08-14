use syntax::ast::CmdStmtNode;
use value::Value;

use crate::{evaluation_error::EvalErr, Evaluable, Interpreter};

impl Evaluable for CmdStmtNode {
    fn evaluate(&self, _state: &mut Interpreter) -> (Value, EvalErr) {
        // TODO add proper parsing of command args based on cmd signature here.
        // Fill those into CommandArgs struct and pass to cmd. For now we do something simple here
        todo!("TODO");
    }
}
