use crate::evaluate::eval_prelude::*;
use lu_interpreter_structs::Function;
use lu_syntax::ast::FnStmtNode;

pub fn eval_function(
    fn_stmt: &Function,
    scope: &mut Arc<Mutex<Scope<Variable>>>,
) -> LuResult<Value> {
    Evaluator::eval_result_to_lu_result(fn_stmt.fn_node.evaluate(scope))
}

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        if let Some(block) = self.block_stmt() {
            match block.evaluate(scope) {
                Err(RetValOrErr::RetVal(v)) => Ok(v),
                v => v,
            }
        } else {
            Ok(Value::Nil)
        }
    }
}
