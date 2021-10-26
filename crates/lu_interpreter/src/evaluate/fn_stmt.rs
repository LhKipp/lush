use crate::evaluate::eval_prelude::*;
use lu_interpreter_structs::Function;
use lu_syntax::ast::FnStmtNode;

pub fn eval_function(fn_stmt: &Function, scope: &mut SyScope) -> LuResult<Value> {
    let result = if let Some(block) = fn_stmt.fn_node.block_stmt() {
        match block.evaluate(scope) {
            Err(RetValOrErr::RetVal(v)) => Ok(v),
            v => v,
        }
    } else {
        Ok(Value::Nil)
    };
    Evaluator::eval_result_to_lu_result(result)
}

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut SyScope) -> EvalResult {
        // Evaluation of fn_stmt happens through the Command trait. (See Function::run)
        Ok(Value::Nil)
    }
}
