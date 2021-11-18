use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::LetStmtNode;

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let var_name = self.var_name().unwrap();
        let val = if let Some(rhs) = self.value() {
            rhs.evaluate(scope)?
        } else {
            Value::Nil
        };

        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            var_name,
            val,
            VarDeclNode::LetStmt(self.clone()),
        ));

        Ok(Value::Nil)
    }

    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
}
