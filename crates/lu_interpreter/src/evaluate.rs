use enum_as_inner::EnumAsInner;
use lu_syntax::ast::HasAstId;
use std::fmt::{Debug, Display};

use log::debug;
use lu_error::{LuErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::{EvalResult, RetValOrErr, SyScope, Value};

mod block_stmt;
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
mod test;

macro_rules! handle_dbg_intervention_before {
    ($dbg_result: ident, $scope: ident) => {{
        match $dbg_result {
            Some(lu_dbg::DbgIntervention::ContinueAsIfStmtRet(val)) => return Ok(val),
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
}

pub trait Evaluable: Display + HasAstId {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[]
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
            let dbg_result =
                lu_dbg::before_eval(&self.to_string().trim(), self.get_ast_id(), scope)?;
            handle_dbg_intervention_before!(dbg_result, scope);
        }

        let result = self.do_evaluate(args, scope);

        if should_stop_for_dbg {
            lu_dbg::after_eval(&self.to_string().trim(), &self.get_ast_id(), scope);
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

    pub fn evaluate(&mut self) {
        // TODO pass node and only eval that
        let node = self
            .scope
            .lock()
            .get_cur_frame()
            .get_tag()
            .as_module_frame()
            .cloned()
            .unwrap()
            .node
            .unwrap();

        let lu_result = Self::eval_result_to_lu_result(node.evaluate(&mut self.scope));
        match lu_result {
            Ok(v) => self.result = Some(v),
            Err(e) => self.errors.push(e),
        }
    }

    pub fn lu_result_to_eval_result<T>(result: LuResult<T>) -> Result<T, RetValOrErr> {
        result.map_err(|e| e.into())
    }

    pub fn eval_result_to_lu_result(result: EvalResult) -> LuResult<Value> {
        result.map_err(|e| match e {
            RetValOrErr::RetVal(v) => {
                unreachable!("Ret val ({:?}) should always be catched by fn_stmt", v)
            }
            RetValOrErr::Err(e) => e,
        })
    }

    // /// Evaluate code as T
    // pub fn evaluate_as<T: Evaluable + HasRule + AstNode>(
    //     code: SourceCode,
    //     &mut self,
    // ) -> Result<Value, RetValOrErr> {
    //     let parse_result = Parse::rule(&code.to_string()?, &*T::get_belonging_rule());
    //     // We don't allow evaluation if errors happend.
    //     let source_file = parse_result.ok::<T>()?;
    //     source_file.evaluate(self)
    // }

    pub(crate) fn as_result(self) -> Result<Value, Vec<LuErr>> {
        if !self.errors.is_empty() {
            Err(self.errors)
        } else {
            Ok(self.result.unwrap())
        }
    }
}
