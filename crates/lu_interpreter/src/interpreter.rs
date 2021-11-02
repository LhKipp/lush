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

impl Default for InterpreterCfg {
    fn default() -> Self {
        InterpreterCfg {
            plugin_dir: "/home/leo/.config/lu/plugins".into(),
        }
    }
}

pub struct Interpreter {
    pub global_frame: ScopeFrame<Variable>,
    pub config: Rc<InterpreterCfg>,
    // TODO rework this whole shitty interpreter construction. Its a pile of crab
    // Scope is set by evaluate. None before :)
    pub scope: Option<SyScope>,
}

impl Interpreter {
    pub fn new(global_frame: ScopeFrame<Variable>, config: InterpreterCfg) -> Self {
        Interpreter {
            config: Rc::new(config),
            global_frame,
            scope: None,
        }
    }

    pub fn parse(&mut self, code: SourceCode) -> Outcome<Parse> {
        Parse::rule(code, &SourceFileRule {})
    }

    pub fn build_scope(
        &mut self,
        parse: Parse,
        global_frame: ScopeFrame<Variable>,
    ) -> Outcome<Scope<Variable>> {
        let relative_include_path_start: PathBuf = std::env::var("PWD").unwrap().into();
        let modules = modules_from_start_parse(
            parse,
            &LoadModulesConfig {
                load_std_module_func: load_std_module,
                plugin_dir: &self.config.plugin_dir,
                relative_include_path_start,
            },
        );

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

    pub fn typecheck(&mut self, scope: Scope<Variable>) -> TyCheckState {
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

    pub fn evaluate(&mut self, ty_state: TyCheckState) -> Option<Evaluator> {
        // We don't allow evaluation if errors happend.
        assert!(ty_state.succeeded());

        let mut evaluator = Evaluator::new(Arc::new(Mutex::new(ty_state.scope.clone())));
        evaluator.evaluate();
        self.scope = Some(evaluator.scope.clone());
        Some(evaluator)
    }

    pub fn eval(&mut self, code: SourceCode) -> LuResults<Value> {
        let ty_check = self.ty_check(code)?;
        self.evaluate(ty_check).unwrap().as_result()
    }

    pub fn ty_check(&mut self, code: SourceCode) -> LuResults<TyCheckState> {
        let parse = self.parse(code);
        let scope = parse.map_flattened(|parse| self.build_scope(parse, self.global_frame.clone()));
        let mut ty_check = scope.map(|scope| self.typecheck(scope));

        let ty_errs = ty_check.val.get_errors().clone();
        ty_check.errs.extend(ty_errs);

        ty_check.into()
    }
}
