use derive_more::From;
use enum_as_inner::EnumAsInner;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use log::debug;
use lu_error::{LuErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::Value;
use lu_pipeline_stage::PipelineStage;
use parking_lot::Mutex;

use crate::{Scope, TyCheckState, Variable};

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
mod test;

pub use fn_stmt::eval_function;

#[derive(Clone, Debug, EnumAsInner)]
pub enum EvalArg {
    ExternalCmdName(String),
    CmdInVal { val: Value, decl: SourceCodeItem },
}

#[derive(Debug, From)]
pub enum RetValOrErr {
    RetVal(Value),
    Err(LuErr),
}

pub type EvalResult = Result<Value, RetValOrErr>;

impl From<RetValOrErr> for EvalResult {
    fn from(e: RetValOrErr) -> Self {
        Err(e)
    }
}

pub trait Evaluable: Display {
    /// Evaluate the AST-Node/Token given the state.
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult;

    fn evaluate(&self, scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        self.evaluate_with_args(&[], scope)
    }

    fn evaluate_with_args(
        &self,
        args: &[EvalArg],
        scope: &mut Arc<Mutex<Scope<Variable>>>,
    ) -> EvalResult {
        debug!("Evaluating: {}", self);
        let result = self.do_evaluate(args, scope);
        debug!("Result of Evaluating: {}: {:?}", self, result);
        result
    }
}

pub struct Evaluator {
    pub ty_state: TyCheckState,

    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,
    /// The final result of this evaluator
    pub result: Option<Value>,
}

impl Evaluator {
    pub fn new(ty_state: TyCheckState) -> Self {
        let scope = ty_state.scope.clone();
        Self {
            ty_state,
            scope: Arc::new(Mutex::new(scope)),
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
            Err(e) => self.push_err(e),
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
        if self.failed() {
            Err(self.collect_all_errors())
        } else {
            Ok(self.result.unwrap())
        }
    }
}

impl PipelineStage for Evaluator {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.ty_state)
    }

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}
