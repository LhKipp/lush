use lu_syntax::ast::SourceFileNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, RetValOrErr};

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> Result<Value, RetValOrErr> {
        let stmts = self.statements().unwrap();
        let result = stmts.evaluate(state)?;
        Ok(result)
    }
}
