use lu_error::LuResult;
use lu_syntax::ast::{FnStmtNode, LetStmtNode, StatementNode};
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for StatementNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        match self {
            StatementNode::LetStmt(n) => n.evaluate(state),
            StatementNode::FnStmt(n) => n.evaluate(state),
            StatementNode::CmdStmt(n) => n.evaluate(state),
            StatementNode::ForStmt(n) => n.evaluate(state),
        }
    }
}

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _state: &mut Interpreter) -> LuResult<Value> {
        todo!()
    }
}

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _state: &mut Interpreter) -> LuResult<Value> {
        todo!()
    }
}
