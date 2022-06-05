use crate::evaluate::eval_prelude::*;
use log::trace;
use lu_interpreter_structs::Function;
use lu_syntax::ast::{BlockStmtNode, FnStmtNode};

pub fn eval_function(fn_stmt: &Function, scope: &mut SyScope) -> LuResult<Value> {
    trace!("Evaluating Function: {}", fn_stmt.name);
    match &fn_stmt.fn_node {
        CmdEvaluableNode::MathExpr(math_expr) => {
            Evaluator::eval_result_to_lu_result(math_expr.evaluate(scope))
        }
        CmdEvaluableNode::FnStmt(fn_node) => eval_fn_cls_block(fn_node.block_stmt(), scope),
        CmdEvaluableNode::ClsExpr(cls_expr) => {
            {
                let mut l_scope = scope.lock();
                for v in &fn_stmt.captured_vars {
                    l_scope.get_cur_frame_mut().insert_var(v.clone());
                }
            }
            return eval_fn_cls_block(cls_expr.block_stmt(), scope);
        }
    }
}

fn eval_fn_cls_block(block: Option<BlockStmtNode>, scope: &mut SyScope) -> LuResult<Value> {
    let result = if let Some(block) = block {
        match block.evaluate(scope) {
            Err(RetValOrErr::RetVal(v)) => Ok(v), // Only retvals are returned
            Ok(_) => Ok(Value::Nil),              // Returned val of block is not returned
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
