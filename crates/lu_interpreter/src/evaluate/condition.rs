#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{ast::ConditionElement, ast::IfStmtNode, AstToken};
use lu_value::Value;

use crate::{EvalArg, EvalResult, Evaluable, Evaluator, Interpreter, RetValOrErr, ScopeFrameTag};

impl Evaluable for ConditionElement {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        match self {
            ConditionElement::CmdStmt(n) => n.evaluate(state),
            ConditionElement::ValueExpr(n) => n.evaluate(state),
        }
    }
}
