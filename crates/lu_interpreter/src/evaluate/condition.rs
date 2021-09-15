#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{ast::ConditionElement, ast::IfStmtNode, AstToken};
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, Interpreter, ScopeFrameTag};

impl Evaluable for ConditionElement {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        match self {
            ConditionElement::CmdStmt(n) => n.evaluate(state),
            ConditionElement::ValueExpr(n) => n.evaluate(state),
        }
    }
}
