use lu_error::LuResult;
use lu_value::Value;

use crate::Interpreter;

mod cmd_call;
mod expr;
mod source_file;
mod statement;

pub trait Evaluable {
    /// Evaluate the AST-Node/Token given the state.
    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value>;
}
