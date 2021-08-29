#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{BlockStmtNode, IfStmtNode},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use lu_value::Value;

use crate::{EvalArg, Evaluable, Interpreter, ScopeFrameTag};

impl Evaluable for IfStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let if_cond = self.if_condition().unwrap();
        let if_block = self.if_block().unwrap();
        let (evaluated, result) = eval_block_if_true(&if_cond, &if_block, state);
        if evaluated || result.is_err() {
            return result;
        }

        for (elif_cond, elif_block) in self.elif_blocks() {
            let elif_cond = elif_cond.unwrap();
            let elif_block = elif_block.unwrap();
            let (evaluated, result) = eval_block_if_true(&elif_cond, &elif_block, state);
            if evaluated || result.is_err() {
                return result;
            }
        }

        if let Some(else_block) = self.else_block() {
            return eval_block(&else_block, state);
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
    state: &mut Interpreter,
) -> (bool, LuResult<Value>) {
    let cond_val = match cond.evaluate(state) {
        Ok(v) => v,
        Err(e) => return (false, Err(e)),
    };

    let cond_val = match cond_val.convert_to_bool() {
        None => {
            return (
                false,
                EvalErr::NotConvertibleToBool(SourceCodeItem::new(
                    cond.syntax().text_range().into(),
                    cond.text(),
                ))
                .into(),
            )
        }
        Some(v) => v,
    };

    if cond_val {
        (true, eval_block(block, state))
    } else {
        (false, Ok(Value::Nil))
    }
}

fn eval_block(block: &BlockStmtNode, state: &mut Interpreter) -> LuResult<Value> {
    state.scope.lock().push_frame(ScopeFrameTag::IfStmtFrame);
    let result = block.evaluate(state);
    state.scope.lock().pop_frame(ScopeFrameTag::IfStmtFrame);
    result
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_syntax::ast::SourceFileNode;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/if_stmt/single_if.json_test")]
    fn single_if_test(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/if_stmt/if_else.json_test")]
    fn if_else_test(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/if_stmt/elif.json_test")]
    fn elif_test(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
