use lu_error::LuResult;
use lu_syntax::ast::StatementElement;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Interpreter};

impl Evaluable for StatementElement {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        match self {
            StatementElement::IfStmt(n) => n.evaluate(state),
            StatementElement::LetStmt(n) => n.evaluate(state),
            StatementElement::FnStmt(n) => n.evaluate(state),
            StatementElement::CmdStmt(n) => n.evaluate(state),
            StatementElement::ForStmt(n) => n.evaluate(state),
            StatementElement::PipedCmdsStmt(n) => n.evaluate(state),
        }
    }
}
