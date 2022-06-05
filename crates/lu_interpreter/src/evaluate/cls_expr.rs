use std::rc::Rc;

use crate::evaluate::eval_prelude::*;
use lu_interpreter_structs::Function;
use lu_syntax::ast::ClosureExprNode;

impl Evaluable for ClosureExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut SyScope) -> EvalResult {
        let cls = Function::closure_from_node(self.clone().into());
        Ok(Value::Command(Rc::new(cls)))
    }
}
