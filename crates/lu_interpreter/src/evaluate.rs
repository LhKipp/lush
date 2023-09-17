use enum_as_inner::EnumAsInner;
use lu_syntax::{
    ast::{HasAstId, HasRule},
    AstNode,
};
use std::fmt::{Debug, Display};

use log::debug;
use lu_error::{LuErr, LuResult, LuResults, SourceCodeItem};
use lu_interpreter_structs::{EvalResult, RetValOrErr, SyScope, Value};

mod block_stmt;
mod cls_expr;
mod cmd_stmt;
mod condition;
mod eval_prelude;
mod expr;
mod fn_stmt;
mod for_stmt;
mod if_stmt;
mod let_stmt;
mod math_expr;
mod piped_cmds_stmt;
mod ret_stmt;
mod source_file;
mod statement;
mod strct_stmt;
mod table_expr;
mod test;

macro_rules! handle_dbg_intervention_before {
    ($dbg_result: ident, $scope: ident) => {{
        log::debug!("Handling dbg intervention {:?}", $dbg_result);
        match $dbg_result {
            Some(lu_dbg::DbgIntervention::ContinueAsIfStmtRet(val_parse)) => {
                // Don't print out evaluated parse
                // TODO save state from before
                lu_interpreter_structs::special_scope_vars::set_silence_stmt_returns(
                    true,
                    $scope.lock().get_cur_frame_mut(),
                );
                let result = match val_parse.sf_node.evaluate($scope) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        todo!("Dbger should only accept correct values: {:?}", e)
                    }
                };
                lu_interpreter_structs::special_scope_vars::set_silence_stmt_returns(
                    false,
                    $scope.lock().get_cur_frame_mut(),
                );
                return result;
            }
            Some(lu_dbg::DbgIntervention::ContinueAsIfStmtRetsNil) => {
                return Ok(lu_interpreter_structs::Value::Nil)
            }
            None => {} // ok
        }
    }};
}
pub(crate) use handle_dbg_intervention_before;

pub use fn_stmt::eval_function;

#[derive(Clone, Debug, EnumAsInner, PartialEq, Eq)]
pub enum EvalArg {
    ExternalCmdName(String),
    CmdInVal { val: Value, decl: SourceCodeItem },
    BlockNoPushFrame,
}

/// The settings a Evaluable node can have
#[derive(Debug, PartialEq, Eq)]
pub enum DbgSetting {
    StopDbgBeforeEval,
    StopDbgBeforeEvalWithNodeText(String),
}

pub trait Evaluable: Display + HasAstId {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[]
    }
    fn dbg_node_text(&self) -> String {
        self.to_string().trim().to_string()
    }

    /// Evaluate the AST-Node/Token given the state.
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut SyScope) -> EvalResult;

    fn evaluate(&self, scope: &mut SyScope) -> EvalResult {
        self.evaluate_with_args(&[], scope)
    }

    fn evaluate_with_args(&self, args: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        debug!("Evaluating: {}", self);

        let should_stop_for_dbg = self.dbg_settings().contains(&DbgSetting::StopDbgBeforeEval);
        if should_stop_for_dbg {
            let dbg_result = lu_dbg::before_eval(&self.dbg_node_text(), self.get_ast_id(), scope)?;
            handle_dbg_intervention_before!(dbg_result, scope);
        }

        let result = self.do_evaluate(args, scope);

        if should_stop_for_dbg {
            lu_dbg::after_eval(&self.get_ast_id(), scope);
        }

        debug!("Result of Evaluating: {}: {:?}", self, result);
        result
    }
}

pub struct Evaluator {
    pub scope: SyScope,
    pub errors: Vec<LuErr>,
    /// The final result of this evaluator
    pub result: Option<Value>,
}

impl Evaluator {
    pub fn new(scope: SyScope) -> Self {
        Self {
            scope,
            errors: Vec::new(),
            result: None,
        }
    }

    pub fn lu_result_to_eval_result<T>(result: LuResult<T>) -> Result<T, RetValOrErr> {
        result.map_err(|e| e.into())
    }

    // Only gives back first err of lu_results
    pub fn lu_results_to_eval_result<T>(result: LuResults<T>) -> Result<T, RetValOrErr> {
        result.map_err(|e| {
            assert!(!e.is_empty());
            e[0].clone().into()
        })
    }

    pub fn eval_result_to_lu_result(result: EvalResult) -> LuResult<Value> {
        result.map_err(|e| match e {
            RetValOrErr::RetVal(v) => {
                unreachable!("Ret val ({:?}) should always be catched by fn_stmt", v)
            }
            RetValOrErr::Err(e) => e,
        })
    }
}
