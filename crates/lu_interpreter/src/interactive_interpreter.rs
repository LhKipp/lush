use lu_error::LuResults;
use lu_interpreter_structs::*;
use lu_parser::grammar::SourceFileRule;
use lu_structure_parse::{modules_from_start_parse, LoadModulesConfig};
use lu_syntax::Parse;
use lu_text_util::SourceCode;
use parking_lot::Mutex;

use std::{rc::Rc, sync::Arc};

use crate::{typecheck::TyCheckState, Evaluable, Evaluator, InterpreterCfg, Scope, Variable};

pub struct InteractiveInterpreter {
    pub config: Rc<InterpreterCfg>,

    ty_checker: TyCheckState,
}

impl InteractiveInterpreter {
    pub fn new(global_frame: ScopeFrame<Variable>, config: InterpreterCfg) -> Self {
        let scope = Self::build_initial_interactive_scope(
            global_frame,
            &config.build_load_modules_config(),
        );

        InteractiveInterpreter {
            config: Rc::new(config),
            ty_checker: TyCheckState::new(scope),
        }
    }

    pub fn eval_line(&mut self, code: &str) -> LuResults<Value> {
        let code: SourceCode = code.into();
        let parse = Parse::rule(code, &SourceFileRule {}).as_results()?;
        let parsed_node = parse.source_file_node();
        let (line_mod_path, mut modules) =
            modules_from_start_parse(parse, &self.config.build_load_modules_config())
                .as_results()?;

        let line_mod = modules
            .iter()
            .position(|module| module.tag.as_module_frame().unwrap().id == line_mod_path)
            .expect("Must work");
        let line_mod = modules.remove(line_mod);

        // Merge the module changes from the current line into the scope
        let cur_frame = self.ty_checker.scope.get_cur_frame_mut();
        assert!(cur_frame.tag.is_module_frame());
        cur_frame.update_cli_module_frame(line_mod);
        for module in modules {
            self.ty_checker.scope.push_sf_frame(module);
        }

        self.ty_checker.typecheck(parsed_node.clone());
        let scope = &mut Arc::new(Mutex::new(self.ty_checker.scope.clone()));

        let result = match Evaluator::eval_result_to_lu_result(parsed_node.evaluate(scope)) {
            Ok(v) => Ok(v),
            Err(e) => Err(vec![e]),
        };

        self.ty_checker.scope = scope.lock().clone();

        result
    }

    fn build_initial_interactive_scope(
        global_frame: ScopeFrame<Variable>,
        load_modules_config: &LoadModulesConfig,
    ) -> Scope<Variable> {
        let code: SourceCode = "".into();
        let parse = Parse::rule(code, &SourceFileRule {});
        let parse = parse.as_results().expect("Empty code never errs");

        let mods = modules_from_start_parse(parse, load_modules_config);
        let (start_mod, modules) = mods.as_results().expect("Empty code never errs");

        let mut scope = Scope::new();
        scope.push_frame_(global_frame);
        for module in modules {
            scope.push_sf_frame(module);
        }
        scope.select_sf_frame(&start_mod).expect("Must work");

        // Erase source_file node from modi, as it is faulty and can't be updated
        // TODO really has to be erased?
        match &mut scope.get_cur_frame_mut().tag {
            ScopeFrameTag::ModuleFrame(modi) => modi.node = None,
            _ => unreachable!(),
        }

        scope
    }
}
