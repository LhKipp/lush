mod is_set;
mod select;
mod ty_of;
use std::rc::Rc;

use lu_interpreter_structs::Command;
use vec_rc::vec_rc;

use crate::builtin::{is_set::IsSetBuiltin, select::SelectBuiltin, ty_of::TyOfBuiltin};

pub fn all_builtin_cmds() -> Vec<Rc<dyn Command>> {
    vec_rc![
        TyOfBuiltin::new(),
        IsSetBuiltin::new(),
        SelectBuiltin::new()
    ]
}
