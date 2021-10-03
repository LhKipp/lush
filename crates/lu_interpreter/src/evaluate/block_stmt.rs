#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{BlockStmtNode, ConditionElement, IfBlockNode},
    ast::{IfStmtNode, StatementElement, StrctStmtNode},
    AstElement, AstToken,
};
use lu_value::Value;

use crate::{
    EvalArg, EvalResult, Evaluable, Evaluator, Interpreter, RetValOrErr, ScopeFrameTag, Variable,
};

impl Evaluable for BlockStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
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
                    state.scope.lock().pop_frame(&ScopeFrameTag::BlockFrame);
                    return Err(e);
                }
            }
        }
        state.scope.lock().pop_frame(&ScopeFrameTag::BlockFrame);
        Ok(result)
    }
}
