use lu_error::LuResult;
use lu_value::Value;
use syntax::ast::SourceFileNode;

use crate::{Evaluable, Interpreter};

impl Evaluable for SourceFileNode {
    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let mut result = Value::Nil;
        for stmt in self.statements() {
            result = stmt.evaluate(state)?;
        }
        Ok(result)
    }
}
