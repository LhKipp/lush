use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

use crate::{EvalArg, EvalResult, Evaluable};
use crate::{Evaluator, RetValOrErr};

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        if let Some(block) = self.block_stmt() {
            match block.evaluate(state) {
                Err(RetValOrErr::RetVal(v)) => Ok(v),
                v => v,
            }
        } else {
            Ok(Value::Nil)
        }
    }
}
