use log::debug;
use lu_error::LuResult;
use lu_syntax::ast::HasSyntaxKind;
use lu_value::Value;

use crate::Interpreter;

mod cmd_stmt;
mod expr;
mod fn_stmt;
mod for_stmt;
mod let_stmt;
mod source_file;
mod statement;

pub trait Evaluable: HasSyntaxKind {
    /// Evaluate the AST-Node/Token given the state.
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value>;

    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        debug!("Evaluating: {:?}", self.get_syntax_kind());
        let result = self.do_evaluate(state);
        debug!(
            "Result of Evaluating: {:?}: {:?}",
            self.get_syntax_kind(),
            result
        );
        result
    }
}
