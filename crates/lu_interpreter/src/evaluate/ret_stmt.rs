use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::RetStmtNode;

impl Evaluable for RetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        Err(RetValOrErr::RetVal(
            self.returned_val().unwrap().evaluate(scope)?,
        ))
    }
}
