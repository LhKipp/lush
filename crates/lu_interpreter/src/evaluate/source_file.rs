use lu_error::LuResult;
use lu_syntax::ast::SourceFileNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let mut result = Value::Nil;
        for stmt in self.statements() {
            result = stmt.evaluate(state)?;
        }
        Ok(result)
    }
}
