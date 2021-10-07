use std::{
    collections::HashSet,
    mem::replace,
    path::{Path, PathBuf},
};

use lu_error::util::Outcome;
use lu_interpreter_structs::{ScopeFrame, UsePath, UsePathVariant, Variable};

/// Load all modules required by the given ScopeFrame, and the modules required by the
/// included modules, and the modules required by these, ... (recursive)
pub fn load_mod_paths(
    start_frame: ScopeFrame<Variable>,
    cfg: LoadModulesConfig,
) -> Outcome<Vec<ScopeFrame<Variable>>> {
    let mut all_frames = vec![start_frame];

    let (id, use_paths) = all_frames[0]
        .get_tag()
        .clone()
        .into_source_file_frame()
        .expect("Arg must be SourceFileFrame");

    let mut sourced_modules = HashSet::new();
    sourced_modules.insert(id);

    // Cant iterate over vec while mutating it?
    let mut paths_to_source = use_paths;

    loop {
        // replace to mutate paths_to_source in for loop
        for use_path in replace(&mut paths_to_source, vec![]) {
            if sourced_modules.contains(&use_path) {
                continue;
            }
            match use_path.ty {
                UsePathVariant::StdPath => all_frames.extend((cfg.load_std_module)(&use_path)),
                UsePathVariant::PluginPath => {
                    todo!("Impl plugin system")
                }
                UsePathVariant::FilePath => {
                    todo!("Impl sourcing of files")
                }
            }
        }
        if paths_to_source.is_empty() {
            break; // No more work
        }
    }

    Outcome::ok(all_frames)
}

pub struct LoadModulesConfig<'a> {
    /// Function for loading a std module
    pub load_std_module: fn(&UsePath) -> Vec<ScopeFrame<Variable>>,
    pub plugin_dir: &'a Path,
    pub relative_include_path_start: PathBuf,
}
