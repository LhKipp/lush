#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{BlockStmtNode, IfStmtNode, StatementElement},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use lu_value::Value;

use crate::{Evaluable, Interpreter, ScopeFrameTag, Variable};

impl Evaluable for BlockStmtNode {
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        // We need to bring all funcs into scope, before running any stmts
        // consider the following program
        // ```lu
        // func_1 arg_1 # func_1 called from below
        //
        // fn func_1 []
        //     ...
        // end
        // ```
        // TODO bringing func decls into scope is only needed for source_file_blocks..., for all
        // others this is a noop
        state.scope.lock().push_frame(ScopeFrameTag::BlockFrame);
        {
            for fn_stmt in self.fn_stmts() {
                fn_stmt.evaluate(state)?;
            }
        }

        let mut result = Value::Nil;

        for stmt in self
            .statements()
            .filter(|stmt| !matches!(stmt, StatementElement::FnStmt(_)))
        {
            match stmt.evaluate(state) {
                Ok(v) => result = v,
                Err(e) => {
                    state.scope.lock().pop_frame(ScopeFrameTag::BlockFrame);
                    return Err(e);
                }
            }
        }
        state.scope.lock().pop_frame(ScopeFrameTag::BlockFrame);
        Ok(result)
    }
}

// #[cfg(test)]
// mod test {
//     use lu_error::LuResult;
//     use lu_syntax::ast::SourceFileNode;
//     use lu_test_support::{init_logger, make_test_interpreter};
//     use lu_text_util::SourceCode;
//     use lu_value::Value;
//     use {conformance, serde_json};

//     #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/if_stmt/single_if.json_test")]
//     fn general_interpreter_tests(s: &str) -> LuResult<Value> {
//         init_logger();
//         let mut itprt = make_test_interpreter();

//         itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
//     }
// }
