mod array;
mod test;
mod iter_funcs;

use lu_error::LuResult;
use lu_interpreter_structs::{Scope, Variable};

pub use array::source_array_module;

/// Source the module specified by path
/// If no such module is found, a error is raised
pub fn source_std(path: &[&str], scope: &mut Scope<Variable>) -> LuResult<()> {
    if path.is_empty() {
        source_array_module(&[], scope)?;
        // Source all

        Ok(())
    } else {
        match path[0] {
            "array" => source_array_module(&path[1..], scope),
            _ => todo!("Return err here"),
        }
    }
}
