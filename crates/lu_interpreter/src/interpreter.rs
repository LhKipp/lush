#![allow(dead_code)]
#![allow(unused_imports)]

use log::debug;
use lu_cmds::load_std_module;
use lu_error::{util::Outcome, LuErr, LuResult, LuResults, ParseErr};
use lu_interpreter_structs::*;
use lu_parser::grammar::SourceFileRule;
use lu_pipeline_stage::PipelineStage;
use lu_structure_parse::{modules_from_start_parse, LoadModulesConfig};
use lu_syntax::{ast::SourceFileNode, AstNode, Parse};
use lu_text_util::SourceCode;

use parking_lot::Mutex;
use std::{path::PathBuf, rc::Rc, sync::Arc};

use crate::{typecheck::TyCheckState, Evaluable, Evaluator, Scope, Variable};

#[derive(Debug)]
pub struct InterpreterCfg {
    pub plugin_dir: PathBuf,
}

impl InterpreterCfg {
    pub fn build_load_modules_config(&self) -> LoadModulesConfig {
        // TODO relative_include_path_start
        let relative_include_path_start: PathBuf = std::env::var("PWD").unwrap().into();
        LoadModulesConfig {
            load_std_module_func: load_std_module,
            plugin_dir: &self.plugin_dir,
            relative_include_path_start,
        }
    }
}

impl InterpreterCfg {
    pub fn try_default() -> LuResult<Self> {
        Ok(InterpreterCfg {
            plugin_dir: lu_cfg_home::plugin_dir()?,
        })
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn eval(scope: &mut SyScope) -> LuResult<Value> {
        // TODO pass node and only eval that
        let node = scope
            .lock()
            .get_cur_frame()
            .get_tag()
            .as_module_frame()
            .cloned()
            .unwrap()
            .node
            .unwrap();

        Evaluator::eval_result_to_lu_result(node.evaluate(scope))
    }

    pub fn ty_check(
        code: SourceCode,
        global_frame: ScopeFrame<Variable>,
        cfg: &InterpreterCfg,
    ) -> Outcome<Scope<Variable>> {
        let parse = Interpreter::parse(code);
        let scope = parse.map_flattened(|parse| Interpreter::build_scope(parse, global_frame, cfg));
        let ty_check = scope.map(|scope| Interpreter::typecheck(scope));

        let mut ty_errs = ty_check.val.get_errors().clone();
        ty_errs.extend(ty_check.errs);

        Outcome::new(ty_check.val.scope, ty_errs)
    }

    pub fn eval_for_tests(
        code: SourceCode,
        global_frame: ScopeFrame<Variable>,
        cfg: &InterpreterCfg,
    ) -> LuResults<Value> {
        let (scope, ty_errs) = Self::ty_check(code, global_frame, cfg).split();
        if !ty_errs.is_empty() {
            return Err(ty_errs);
        }
        match Self::eval(&mut Arc::new(Mutex::new(scope))) {
            Ok(v) => Ok(v),
            Err(e) => Err(vec![e]),
        }
    }

    pub fn ty_check_for_tests(
        code: SourceCode,
        global_frame: ScopeFrame<Variable>,
        cfg: &InterpreterCfg,
    ) -> LuResults<Option<ValueType>> {
        let parse = Interpreter::parse(code);
        let scope = parse.map_flattened(|parse| Interpreter::build_scope(parse, global_frame, cfg));
        let ty_check = scope.map(|scope| Interpreter::typecheck(scope));

        let mut ty_errs = ty_check.val.get_errors().clone();
        ty_errs.extend(ty_check.errs);

        if ty_errs.is_empty() {
            Ok(ty_check.val.result)
        } else {
            Err(ty_errs)
        }
    }

    fn parse(code: SourceCode) -> Outcome<Parse> {
        Parse::rule(code, &SourceFileRule {})
    }

    fn build_scope(
        parse: Parse,
        global_frame: ScopeFrame<Variable>,
        cfg: &InterpreterCfg,
    ) -> Outcome<Scope<Variable>> {
        let load_modules_config = cfg.build_load_modules_config();
        let modules = modules_from_start_parse(parse, &load_modules_config);

        modules.map(move |(start_mod, modules)| {
            let mut scope = Scope::new();
            scope.push_frame_(global_frame);
            for module in modules {
                scope.push_sf_frame(module);
            }
            scope.select_sf_frame(&start_mod).expect("Must work");

            scope
        })
    }

    /// Typecheck starting at the currently selected mod frame
    fn typecheck(scope: Scope<Variable>) -> TyCheckState {
        let mut ty_state = TyCheckState::new(scope);
        let sf_node = ty_state
            .scope
            .get_cur_frame()
            .get_tag()
            .as_module_frame()
            .cloned()
            .unwrap()
            .node
            .unwrap();
        ty_state.typecheck(sf_node);
        ty_state
    }
}
