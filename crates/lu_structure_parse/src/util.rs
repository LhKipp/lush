use std::path::{Path, PathBuf};

use lu_error::{util::Outcome, SourceCodeItem};
use lu_interpreter_structs::{ModPath, ScopeFrame, Variable};

pub struct LoadModulesConfig<'a> {
    /// Function for loading a std module
    pub load_std_module_func: fn(&ModPath, &SourceCodeItem) -> Outcome<Vec<ScopeFrame<Variable>>>,
    pub plugin_dir: &'a Path,
    pub pwd: PathBuf,
}
