mod array;
mod iter_funcs;
mod test;

use log::debug;
use lu_interpreter_structs::{ScopeFrame, UsePath, UsePathVariant, Variable};

pub use array::source_array_module;

/// Source the module specified by path
/// If no such module is found, a error is raised
pub fn load_std_module(path: &UsePath) -> Vec<ScopeFrame<Variable>> {
    assert!(path.ty == UsePathVariant::StdPath);
    debug!("load_std_module: {}", path);
    if path.parts.len() == 1 {
        // Source all
        // source_array_module(&[], scope);
        todo!();
    } else {
        match path.parts[1].as_ref() {
            "array" => source_array_module(&path.parts[2..]),
            _ => todo!(),
        }
    }
}
