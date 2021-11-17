mod ty_of;
use std::rc::Rc;

use lu_interpreter_structs::Command;
use vec_rc::vec_rc;

use crate::builtin::ty_of::TyOfBuiltin;

pub fn all_builtin_cmds() -> Vec<Rc<dyn Command>> {
    vec_rc![TyOfBuiltin::new()]
}
