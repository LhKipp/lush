use crate::{Scope, Variable};

pub fn is_dbg_session(scope: &Scope<Variable>) -> bool {
    scope.find_var("DBG_SESSION").is_some()
}
