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

use crate::{Evaluable, Interpreter, ScopeFrameTag};

impl Evaluable for BlockStmtNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let result = Value::Nil;
        for stmt in self.statements() {
            result = stmt.evaluate(state)?
        }
        Ok(result)
    }
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
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
