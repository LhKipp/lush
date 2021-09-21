use lu_error::LuResult;
use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

use crate::Evaluator;
#[allow(unused_imports)]
use crate::{Callable, EvalArg, Evaluable, Function, Interpreter, Variable};

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut Evaluator) -> LuResult<Value> {
        Ok(Value::Nil)
    }
}
