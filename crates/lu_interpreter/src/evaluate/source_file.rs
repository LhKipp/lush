use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::SourceFileNode;

impl Evaluable for SourceFileNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let stmts = self.block().unwrap();
        match stmts.evaluate_with_args(&[EvalArg::BlockNoPushFrame], scope) {
            Err(RetValOrErr::RetVal(v)) => Ok(v),
            v => v,
        }
    }
}
