use std::{
    collections::HashSet,
    mem::replace,
    path::{Path, PathBuf},
};

use log::debug;
use lu_error::util::Outcome;
use lu_interpreter_structs::{ModPath, ModPathVariant, ScopeFrame, Variable};
use lu_parser::grammar::SourceFileRule;
use lu_syntax::{ast::SourceFileNode, Parse};
use lu_text_util::SourceCode;
use walkdir::WalkDir;

use crate::source_node_to_scope_frame;

/// Load all modules required by the given ScopeFrame, and the modules required by the
/// included modules, and the modules required by these, ... (recursive)
pub fn load_mod_paths(
    start_frame: ScopeFrame<Variable>,
    cfg: LoadModulesConfig,
) -> Outcome<Vec<ScopeFrame<Variable>>> {
    let mut all_frames = vec![start_frame];
    let mut errs = vec![];

    let (id, use_paths) = all_frames[0]
        .get_tag()
        .clone()
        .into_sf_frame()
        .expect("Arg must be SourceFileFrame");

    let mut sourced_modules = HashSet::new();
    sourced_modules.insert(id);

    let mut paths_to_source = use_paths;

    loop {
        // replace to mutate paths_to_source in for loop
        for use_path in replace(&mut paths_to_source, vec![]) {
            if sourced_modules.contains(&use_path) {
                continue;
            }
            debug!("Loading module: {}", use_path);
            match use_path.variant {
                ModPathVariant::StdPath => all_frames.extend((cfg.load_std_module)(&use_path)),
                ModPathVariant::PluginPath => {
                    let plug_f_path = use_path.as_f_path();
                    let plug_f_path = cfg.plugin_dir.join(plug_f_path);

                    debug!("Loading plug-mod: {:?}", plug_f_path);
                    for f_entry in WalkDir::new(plug_f_path).into_iter() {
                        let file = f_entry.expect("TODO").into_path();
                        debug!("Loading mod: {:?}", file);
                        match SourceCode::new_file(file.clone()) {
                            Err(e) => {
                                debug!("Loading of plug-mod with f_path: {:?} failed", file);
                                errs.push(e);
                            }
                            Ok(source_code) => {
                                let parse = Parse::rule(source_code, &SourceFileRule {});
                                assert!(parse.errors.is_empty()); // TODO make it a outcome
                                let (module, new_mod_err) = source_node_to_scope_frame(
                                    &parse.cast::<SourceFileNode>().unwrap(),
                                    use_path.clone(),
                                )
                                .split();
                                errs.extend(new_mod_err);
                                all_frames.push(module);
                            }
                        }
                    }
                }
                ModPathVariant::FilePath => {
                    todo!("Impl sourcing of files")
                }
            }
            sourced_modules.insert(use_path);
        }
        if paths_to_source.is_empty() {
            break; // No more work
        }
    }

    Outcome::ok(all_frames)
}

pub struct LoadModulesConfig<'a> {
    /// Function for loading a std module
    pub load_std_module: fn(&ModPath) -> Vec<ScopeFrame<Variable>>,
    pub plugin_dir: &'a Path,
    pub relative_include_path_start: PathBuf,
}
