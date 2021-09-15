#![allow(dead_code)]
#![allow(unused_imports)]

use lu_error::{LuErr, LuResult, ParseErr};
use lu_parser::grammar::SourceFileRule;
use lu_syntax::{
    ast::{HasRule, SourceFileNode},
    AstNode, Parse,
};
use lu_text_util::SourceCode;
use lu_value::Value;

use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::{typecheck::TypeChecker, Evaluable, Evaluator, Resolver, Scope, Variable};

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

    pub fn run(&mut self, code: SourceCode) -> Result<Value, Vec<LuErr>> {
        let mut errs = Vec::new();
        let (code, source_name) = match code.unpack() {
            Ok(c) => c,
            Err(e) => return Err(vec![e]),
        };
        let source_name = source_name.unwrap_or("tmp_text".into());

        let parse_result = Parse::rule(&code, &SourceFileRule {});
        let source_file = parse_result.cast::<SourceFileNode>().unwrap();
        errs.extend(parse_result.errors.into_iter().map(|e| LuErr::from(e)));

        let mut resolver = Resolver::new(self.scope.clone());
        resolver.resolve(
            &source_file,
            source_name.into_os_string().into_string().unwrap(),
        );
        errs.extend(resolver.errors);

        let mut ty_checker = TypeChecker::new(resolver.scope);
        ty_checker.typecheck(&source_file);
        errs.extend(ty_checker.errors);

        // We don't allow evaluation if errors happend.
        if !errs.is_empty() {
            return Err(errs);
        }

        let mut evaluator = Evaluator::new(ty_checker.scope);
        evaluator.evaluate(&source_file);

        if evaluator.succeeded() {
            Ok(evaluator.result.unwrap())
        } else {
            errs.extend(evaluator.errors);
            Err(errs)
        }
    }

    pub fn evaluate(&mut self, code: SourceCode) -> Result<Value, Vec<LuErr>> {
        self.run(code)
    }
}
