use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::SourceFileNode;

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let stmts = self.statements().unwrap();
        let result = stmts.evaluate(scope)?;
        Ok(result)
    }
}
