use lu_error::LuResults;
use lu_interpreter_structs::*;
use lu_structure_parse::{modules_from_start_parse, LoadModulesConfig};
use lu_syntax::Parse;
use lu_text_util::SourceCode;
use parking_lot::Mutex;

use std::{mem, rc::Rc, sync::Arc};

use crate::{typecheck::TyCheckState, Evaluable, Evaluator, InterpreterCfg, Scope, Variable};

pub struct InteractiveInterpreter {
    pub config: Rc<InterpreterCfg>,

    pub ty_checker: TyCheckState,
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

    fn get_cli_modi(&self) -> &ModInfo {
        self.ty_checker
            .scope
            .get_all_frames()
            .filter_map(|frame| frame.get_tag().as_module_frame())
            .find_map(|module| {
                if module.id.as_interactive().is_some() {
                    Some(module)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn eval_line(&mut self, code: &str) -> LuResults<Value> {
        let cli_modi = self.get_cli_modi();
        let parse = Parse::cli_line(code.into(), (cli_modi.src.text.len() as u32).into());
        let parsed_node = parse.val.sf_node.clone();
        // We can only display the errors after all modules have been updated in the scope
        let (modules, errs) = parse
            .map_flattened(|parse| {
                modules_from_start_parse(parse, &self.config.build_load_modules_config())
            })
            .split();
        let (line_mod_path, mut modules) = modules;

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
            // TODO only push if is not yet present
            self.ty_checker.scope.push_sf_frame(module);
        }

        // After merging of the modules into the scope, we can display the errors
        if !errs.is_empty() {
            return Err(errs);
        }

        self.ty_checker.typecheck(parsed_node.clone());
        if !self.ty_checker.errors.is_empty() {
            return Err(mem::replace(&mut self.ty_checker.errors, vec![]));
        }

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
        let parse = Parse::source_file(code);
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
