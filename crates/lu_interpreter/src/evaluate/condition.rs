use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::ConditionElement;

impl Evaluable for ConditionElement {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            ConditionElement::PipedCmdsStmt(n) => n.evaluate(scope),
            ConditionElement::ValueExpr(n) => n.evaluate(scope),
            ConditionElement::CmdStmt(n) => n.evaluate(scope),
        }
    }
}
