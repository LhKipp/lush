use crate::{evaluate::eval_prelude::*, handle_dbg_intervention_before};
use lu_syntax::{
    ast::ConditionElement,
    ast::{BlockStmtNode, HasAstId, IfStmtNode},
};

impl Evaluable for IfStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let if_cond = self.if_condition().unwrap();
        let if_block = self.if_block().unwrap();

        let dbg_result = lu_dbg::before_eval(
            &format!("if {}", if_cond.to_string().trim()),
            self.get_ast_id(),
            scope,
        )?;
        // TODO no handle_dbg_intervention_before would be necessary if "if_stmt" gets parsed as
        // IfElifElseStmt -- Has dbg_before not set
        //   IfStmt -- Has dbg_before set
        //   ElifStmt -- Has dbg_before set
        //   ElseStmt -- Has dbg_before set
        handle_dbg_intervention_before!(dbg_result, scope);

        let (evaluated, result) = eval_block_if_true(&if_cond, &if_block, scope);
        if evaluated || result.is_err() {
            return result;
        }

        for (elif_cond, elif_block) in self.elif_blocks() {
            let elif_cond = elif_cond.unwrap();
            let elif_block = elif_block.unwrap();

            let dbg_result = lu_dbg::before_eval(
                &format!("elif {}", elif_cond.text_trimmed()),
                self.get_ast_id(),
                scope,
            )?;
            handle_dbg_intervention_before!(dbg_result, scope);

            let (evaluated, result) = eval_block_if_true(&elif_cond, &elif_block, scope);
            if evaluated || result.is_err() {
                return result;
            }
        }

        if let Some(else_block) = self.else_block() {
            let dbg_result = lu_dbg::before_eval("else", self.get_ast_id(), scope)?;
            handle_dbg_intervention_before!(dbg_result, scope);
            return eval_block(&else_block, scope);
        }

        Ok(Value::Nil)
    }
}

/// Eval `block` if `cond` evaluates to true
/// Returns (true, result) if block has been evaluated
/// Returns (false, result) if block has not been evaluated (result can still contain error)
///
/// (The value of v in return (false, Ok(v)) is unspecified)
fn eval_block_if_true(
    cond: &ConditionElement,
    block: &BlockStmtNode,
    scope: &mut SyScope,
) -> (bool, EvalResult) {
    let cond_val = match cond.evaluate(scope) {
        Ok(v) => v,
        Err(e) => return (false, Err(e)),
    };

    let cond_val = match cond_val.convert_to_bool() {
        None => {
            return (
                false,
                Err(LuErr::Eval(EvalErr::NotConvertibleToBool(cond.to_item())).into()),
            )
        }
        Some(v) => v,
    };

    if cond_val {
        (true, eval_block(block, scope))
    } else {
        (false, Ok(Value::Nil))
    }
}

fn eval_block(block: &BlockStmtNode, scope: &mut SyScope) -> EvalResult {
    scope.lock().push_frame(ScopeFrameTag::IfStmtFrame);
    let result = block.evaluate_with_args(&[EvalArg::BlockNoPushFrame], scope);
    scope.lock().pop_frame(&ScopeFrameTag::IfStmtFrame);
    result
}
