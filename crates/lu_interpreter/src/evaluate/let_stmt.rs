use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::LetStmtNode;

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let var_name = self.var_name().unwrap();
        let val = self.value().unwrap();

        let val = val.evaluate(scope)?;

        scope.lock().get_cur_frame_mut().insert_var(Variable::new(
            var_name,
            val,
            VarDeclNode::LetStmt(self.clone()),
        ));

        Ok(Value::Nil)
    }
}
