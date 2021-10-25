use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::ConditionElement;

impl Evaluable for ConditionElement {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        match self {
            ConditionElement::CmdStmt(n) => n.evaluate(scope),
            ConditionElement::ValueExpr(n) => n.evaluate(scope),
        }
    }

    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval, DbgSetting::StopDbgAfterEval]
    }
}
