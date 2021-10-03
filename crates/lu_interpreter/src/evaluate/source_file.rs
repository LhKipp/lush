use lu_syntax::ast::SourceFileNode;

use crate::{EvalArg, EvalResult, Evaluable, Evaluator};

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        let stmts = self.statements().unwrap();
        let result = stmts.evaluate(state)?;
        Ok(result)
    }
}
