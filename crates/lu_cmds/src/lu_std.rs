mod array;
mod fs;
mod iter_funcs;
mod lu_native_std_mod;
mod test;

use log::debug;
use lu_error::{util::Outcome, AstErr, SourceCodeItem};
use lu_interpreter_structs::{ModPath, ScopeFrame, Variable};
pub(crate) use lu_native_std_mod::{LuNativeStdMod, LuRustStdMod, LuStdMod};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::lu_std::{array::StdArrayMod, fs::StdFsMod};

use self::iter_funcs::IterFuncsMod;

static STD_MODULES: Lazy<HashMap<String, LuStdMod>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let std_mods: Vec<LuStdMod> = vec![
        LuStdMod::Native(Box::new(IterFuncsMod {})),
        LuStdMod::Rust(Box::new(StdArrayMod {})),
        LuStdMod::Rust(Box::new(StdFsMod {})),
    ];
    for std_mod in std_mods.into_iter() {
        map.insert(std_mod.id(), std_mod);
    }
    map
});

/// Source the module specified by path
// TODO error if no such module
pub fn load_std_module(
    path: &ModPath,
    path_usage: &SourceCodeItem,
) -> Outcome<Vec<ScopeFrame<Variable>>> {
    if let Some(path) = path.as_std_path() {
        debug!("load_std_module: {}", path);
        if let Some(module) = STD_MODULES.get(path) {
            vec![module.frame()].into()
        } else {
            Outcome::new(
                vec![],
                vec![AstErr::NoSuchStdPath {
                    path: path.clone(),
                    path_usage: path_usage.clone(),
                }
                .into()],
            )
        }
    } else {
        todo!("Impl error for path not std_path");
    }
}
