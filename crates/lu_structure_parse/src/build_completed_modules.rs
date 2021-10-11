use lu_error::util::Outcome;
use lu_interpreter_structs::*;
use lu_syntax::Parse;

use crate::{load_mod_paths, LoadModulesConfig};

/// Returns (start_id, modules)
pub fn modules_from_start_parse(
    start_parse: Parse,
    cfg: &LoadModulesConfig,
) -> Outcome<(ModPath, Vec<ScopeFrame<Variable>>)> {
    // Step 1: convert given parse to frame
    let start_mod = ModInfo::module_from_parse(start_parse, cfg.plugin_dir);
    let start_mod_path = start_mod
        .val
        .get_tag()
        .as_module_frame()
        .unwrap()
        .id
        .clone();

    // Step 2: load all modules required by a (start)-frame (recursive)
    let modules = start_mod.map_flattened(|start_mod| load_mod_paths(start_mod, cfg));

    // Step 3: Convert ValueType::StrctName to ValueType::Strct
    // TODO

    // Step 4:
    // General purpose resolution?
    // Result could be resolution table: NodeId => ResolutionElem
    // where ResoultionElem is var referenced by VarPath
    // or Cmd called by CommandStmt
    // ???

    // TODO make this a pure func and remove the struct wrapper
    modules.map(|modules| (start_mod_path, modules))
}
