use log::debug;
use lu_error::lu_source_code_item;
use lu_error::SourceCodeItem;

use crate::Value;
use crate::{Scope, ScopeFrame, Variable};

const DBG_SESSION_VAR_NAME: &str = "DBG_SESSION";
pub fn is_dbg_session(scope: &Scope<Variable>) -> bool {
    scope
        .find_var(DBG_SESSION_VAR_NAME)
        .map(|var| match var.val {
            Value::Bool(v) => v,
            _ => false,
        })
        .unwrap_or(false)
}

pub fn set_dbg_session(frame: &mut ScopeFrame<Variable>) {
    debug!("Inserting DBG_SESSION=true");
    frame.insert_var(Variable::new(
        DBG_SESSION_VAR_NAME.to_string(),
        true.into(),
        lu_source_code_item!().into(),
    ));
}
