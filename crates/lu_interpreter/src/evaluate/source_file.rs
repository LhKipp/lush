use lu_error::LuResult;
use lu_syntax::ast::SourceFileNode;
use lu_value::Value;

use crate::{Evaluable, Interpreter, ScopeFrameTag};

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let mut result = Value::Nil;
        state.scope.lock().push_frame(ScopeFrameTag::GlobalFrame);
        for stmt in self.statements() {
            result = stmt.evaluate(state)?;
        }
        state.scope.lock().pop_frame(ScopeFrameTag::GlobalFrame);
        Ok(result)
    }
}
