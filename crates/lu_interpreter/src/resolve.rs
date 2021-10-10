mod test;
use lu_cmds::load_std_module;
use lu_interpreter_structs::ModPath;
use lu_interpreter_structs::ModuleInfo;
use lu_structure_parse::{load_mod_paths, LoadModulesConfig};
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use lu_error::LuErr;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::SourceFileNode;
use lu_syntax::Parse;

use crate::visit_arg::VisitArg;
use crate::{InterpreterCfg, Scope, Variable};

#[derive(Educe)]
#[educe(Debug)]
pub struct Resolver {
    pub parses: Vec<Parse>,
    #[educe(Debug(ignore))]
    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,

    pub config: Rc<InterpreterCfg>,
}

#[derive(Clone, Debug)]
pub enum ResolveArg {
    Arg(VisitArg),
}

impl Resolver {
    pub fn new(
        parse: Parse,
        scope: Arc<Mutex<Scope<Variable>>>,
        config: Rc<InterpreterCfg>,
    ) -> Self {
        Self {
            parses: vec![parse],
            scope,
            errors: Vec::new(),
            config,
        }
    }

    pub(crate) fn resolve(&mut self) {
        let source_file = self.get_start_parse().cast::<SourceFileNode>().unwrap();
        let source_f_path = &self.get_start_parse().source.path;
        let src = self.get_start_parse().source.clone();

        // Parsing happens in fs-path world
        // Resolving happens in UsePath world. Therefore we convert the f_path
        let source_f_path = ModPath::new_start_path(source_f_path);

        // Step 1: convert given file to frame
        let (source_f_frame, errs1) =
            ModuleInfo::module_from_source_node(source_file, source_f_path.clone(), src).split();
        // Step 2: load all modules required by a (start)-frame (recursive)
        // TODO get pwd from scope
        let pwd = std::env::var("PWD").unwrap().into();
        let (frames, errs2) = load_mod_paths(
            source_f_frame,
            LoadModulesConfig {
                load_std_module,
                plugin_dir: self.config.plugin_dir.as_ref(),
                relative_include_path_start: pwd,
            },
        )
        .split();

        // Step 3: Convert ValueType::StrctName to ValueType::Strct

        // Step 4:
        // General purpose resolution?
        // Result could be resolution table: NodeId => ResolutionElem
        // where ResoultionElem is var referenced by VarPath
        // or Cmd called by CommandStmt
        // ???

        // TODO make this a pure func and remove the struct wrapper
        for frame in frames {
            self.scope.lock().push_sf_frame(frame)
        }
        self.scope.lock().select_sf_frame(&source_f_path).unwrap();
        self.push_errs(errs1);
        self.push_errs(errs2);
    }

    pub fn get_start_parse(&self) -> &Parse {
        &self.parses[0]
    }
}

impl PipelineStage for Resolver {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.parses[0])
    }

    fn collect_all_errors_cb(&self) -> Vec<LuErr> {
        self.parses
            .iter()
            .map(|parse| parse.get_errors().clone())
            .flatten()
            .collect()
    }

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}
