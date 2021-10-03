use log::warn;
use lu_error::LuResult;
use lu_syntax::ast::RetStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator};

impl Evaluable for RetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], _state: &mut Evaluator) -> LuResult<Value> {
        warn!("RET STMT EVAL NOT YET IMPL");
        Ok(Value::Nil)
    }
}
