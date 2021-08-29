use std::fmt::Debug;

use log::debug;
use lu_error::LuResult;
use lu_value::Value;

use crate::Interpreter;

mod block_stmt;
mod cmd_stmt;
mod condition;
mod expr;
mod fn_stmt;
mod for_stmt;
mod if_stmt;
mod let_stmt;
mod math_expr;
mod piped_cmds_stmt;
mod source_file;
mod statement;

#[derive(Clone, Debug)]
pub enum EvalArg {
    ExternalCmdName(String),
}

pub trait Evaluable: Debug {
    /// Evaluate the AST-Node/Token given the state.
    fn do_evaluate(&self, args: &[EvalArg], state: &mut Interpreter) -> LuResult<Value>;

    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        self.evaluate_with_args(&[], state)
    }

    fn evaluate_with_args(&self, args: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        debug!("Evaluating: {:?}({:?})", self, args);
        let result = self.do_evaluate(args, state);
        debug!("Result of Evaluating: {:?}({:?}): {:?}", self, args, result);
        result
    }
}
