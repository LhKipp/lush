mod array;
mod iter_funcs;
mod lu_native_std_mod;
mod test;

use log::debug;
use lu_interpreter_structs::{ModPath, ScopeFrame, Variable};
pub(crate) use lu_native_std_mod::{LuNativeStdMod, LuRustStdMod, LuStdMod};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::lu_std::array::StdArrayMod;

use self::iter_funcs::IterFuncsMod;

static STD_MODULES: Lazy<HashMap<String, LuStdMod>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let std_mods: Vec<LuStdMod> = vec![
        LuStdMod::Native(Box::new(IterFuncsMod {})),
        LuStdMod::Rust(Box::new(StdArrayMod {})),
    ];
    for std_mod in std_mods.into_iter() {
        map.insert(std_mod.id(), std_mod);
    }
    map
});

/// Source the module specified by path
// TODO error if no such module
pub fn load_std_module(path: &ModPath) -> Vec<ScopeFrame<Variable>> {
    let path = path.as_std_path().expect("ModPath must be stdpath");
    debug!("load_std_module: {}", path);
    debug!("{:?}", STD_MODULES.keys().collect::<Vec<_>>());
    let module = STD_MODULES.get(path).expect("TODO");
    vec![module.frame()]
}
