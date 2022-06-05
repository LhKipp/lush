use crate::evaluate::eval_prelude::*;
use lu_interpreter_structs::Function;
use lu_syntax::ast::{BlockStmtNode};

impl Evaluable for ClsExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut SyScope) -> EvalResult {
        // Evaluation of fn_stmt happens through the Command trait. (See Function::run)
        Ok(Value::Nil)
    }
}
