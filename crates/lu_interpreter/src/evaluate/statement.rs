use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::StatementElement;

impl Evaluable for StatementElement {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        match self {
            StatementElement::IfStmt(n) => n.evaluate(scope),
            StatementElement::LetStmt(n) => n.evaluate(scope),
            StatementElement::FnStmt(n) => n.evaluate(scope),
            StatementElement::CmdStmt(n) => n.evaluate(scope),
            StatementElement::ForStmt(n) => n.evaluate(scope),
            StatementElement::PipedCmdsStmt(n) => n.evaluate(scope),
            StatementElement::ValueExpr(n) => n.evaluate(scope),
            StatementElement::RetStmt(n) => n.evaluate(scope),
        }
    }
}
