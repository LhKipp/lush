use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::RetStmtNode;

impl Evaluable for RetStmtNode {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        Err(RetValOrErr::RetVal(
            self.returned_val().unwrap().evaluate(scope)?,
        ))
    }
}
