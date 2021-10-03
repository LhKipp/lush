use lu_syntax::ast::RetStmtNode;

use crate::{EvalArg, EvalResult, Evaluable, Evaluator, RetValOrErr};

impl Evaluable for RetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        Err(RetValOrErr::RetVal(
            self.returned_val().unwrap().evaluate(state)?,
        ))
    }
}
