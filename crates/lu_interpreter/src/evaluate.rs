use std::{fmt::Debug, sync::Arc};

use log::debug;
use lu_error::{LuErr, LuResult};
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::SourceFileNode;
use lu_value::Value;
use parking_lot::Mutex;

use crate::{Scope, TypeChecker, Variable};

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
mod test;

#[derive(Clone, Debug)]
pub enum EvalArg {
    ExternalCmdName(String),
}

pub trait Evaluable: Debug {
    /// Evaluate the AST-Node/Token given the state.
    fn do_evaluate(&self, args: &[EvalArg], state: &mut Evaluator) -> LuResult<Value>;

    fn evaluate(&self, state: &mut Evaluator) -> LuResult<Value> {
        self.evaluate_with_args(&[], state)
    }

    fn evaluate_with_args(&self, args: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        debug!("Evaluating: {:?}({:?})", self, args);
        let result = self.do_evaluate(args, state);
        debug!("Result of Evaluating: {:?}({:?}): {:?}", self, args, result);
        result
    }
}

pub struct Evaluator {
    pub ty_checker: TypeChecker,

    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,
    /// The final result of this evaluator
    pub result: Option<Value>,
}

impl Evaluator {
    pub fn new(ty_checker: TypeChecker) -> Self {
        let scope = ty_checker.resolve.scope.clone();
        Self {
            ty_checker,
            scope,
            errors: Vec::new(),
            result: None,
        }
    }

    pub fn evaluate(&mut self) {
        let node = self
            .ty_checker
            .resolve
            .parse
            .cast::<SourceFileNode>()
            .unwrap();
        match node.evaluate(self) {
            Ok(v) => self.result = Some(v),
            Err(e) => self.errors.push(e),
        }
    }

    // /// Evaluate code as T
    // pub fn evaluate_as<T: Evaluable + HasRule + AstNode>(
    //     &mut self,
    //     code: SourceCode,
    // ) -> LuResult<Value> {
    //     let parse_result = Parse::rule(&code.to_string()?, &*T::get_belonging_rule());
    //     // We don't allow evaluation if errors happend.
    //     let source_file = parse_result.ok::<T>()?;
    //     source_file.evaluate(self)
    // }

    pub fn failed(&self) -> bool {
        !self.errors.is_empty()
    }

    pub(crate) fn all_errors(&self) -> Vec<LuErr> {
        let mut errs = self.ty_checker.collect_all_errors();
        errs.extend(self.errors.clone());
        errs
    }

    pub(crate) fn as_result(self) -> Result<Value, Vec<LuErr>> {
        if self.failed() {
            Err(self.all_errors())
        } else {
            Ok(self.result.unwrap())
        }
    }
}
