use lu_syntax::ast::RetStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, RetValOrErr};

impl Evaluable for RetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> Result<Value, RetValOrErr> {
        Err(RetValOrErr::RetVal(
            self.returned_val().unwrap().evaluate(state)?,
        ))
    }
}
