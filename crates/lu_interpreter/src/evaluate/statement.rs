use lu_error::LuResult;
use lu_syntax::ast::StatementNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for StatementNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        match self {
            StatementNode::IfStmt(n) => n.evaluate(state),
            StatementNode::LetStmt(n) => n.evaluate(state),
            StatementNode::FnStmt(n) => n.evaluate(state),
            StatementNode::CmdStmt(n) => n.evaluate(state),
            StatementNode::ForStmt(n) => n.evaluate(state),
        }
    }
}
