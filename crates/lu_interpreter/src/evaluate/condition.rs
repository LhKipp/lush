use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::ConditionElement;

impl Evaluable for ConditionElement {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            ConditionElement::CmdStmt(n) => n.evaluate(scope),
            ConditionElement::ValueExpr(n) => n.evaluate(scope),
        }
    }
}
