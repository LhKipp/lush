#![allow(dead_code)]
#![allow(unused_imports)]

use lu_error::{LuErr, LuResult, LuResults, ParseErr};
use lu_parser::grammar::SourceFileRule;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{
    ast::{HasRule, SourceFileNode},
    AstNode, Parse,
};
use lu_text_util::SourceCode;
use lu_value::Value;

use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::{typecheck::TyCheckState, Evaluable, Evaluator, Resolver, Scope, Variable};

/// The interpreter holds data (scope), getting transformed while interpreting the ast.
pub struct Interpreter {
    pub scope: Arc<Mutex<Scope<Variable>>>,
}

impl Interpreter {
    pub fn new(scope: Scope<Variable>) -> Self {
        Interpreter {
            scope: Arc::new(Mutex::new(scope)),
        }
    }

    pub fn parse(&mut self, code: SourceCode) -> Parse {
        Parse::rule(code, &SourceFileRule {})
    }

    pub fn resolve(&mut self, parse: Parse) -> Resolver {
        let mut resolver = Resolver::new(parse, self.scope.clone());
        resolver.resolve();

        resolver
    }

    pub fn typecheck(&mut self, resolve: Resolver) -> TyCheckState {
        let mut ty_state = TyCheckState::new(resolve);
        ty_state.typecheck();
        ty_state
    }

    pub fn evaluate(&mut self, ty_state: TyCheckState) -> Option<Evaluator> {
        // We don't allow evaluation if errors happend.
        if ty_state.failed() {
            return None;
        }

        let mut evaluator = Evaluator::new(ty_state);
        evaluator.evaluate();
        Some(evaluator)

        // if evaluator.succeeded() {
        //     Ok(evaluator.result.unwrap())
        // } else {
        //     Err(evaluator.all_errors())
        // }
    }

    pub fn eval(&mut self, code: SourceCode) -> LuResults<Value> {
        let parse = self.parse(code);
        let resolve = self.resolve(parse);
        let ty_check = self.typecheck(resolve);
        let ty_check = ty_check.as_result()?;
        self.evaluate(ty_check).unwrap().as_result()
    }

    pub fn ty_check(&mut self, code: SourceCode) -> LuResults<TyCheckState> {
        let parse = self.parse(code);
        let resolve = self.resolve(parse);
        let ty_check = self.typecheck(resolve);
        ty_check.as_result()
    }
}
