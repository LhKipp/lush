use log::debug;
use lu_error::lu_source_code_item;
use lu_stdx::{new_amtx, AMtx};

use crate::dbg_state::DbgState;
use crate::Value;
use crate::{Scope, ScopeFrame, Variable};

const DBG_SESSION_VAR_NAME: &str = "DBG_SESSION";
pub fn get_dbg_session(scope: &Scope<Variable>) -> Option<&AMtx<DbgState>> {
    scope
        .find_var(DBG_SESSION_VAR_NAME)
        .map(|var| match &var.val {
            Value::DbgState(s) => Some(s),
            _ => None,
        })
        .flatten()
}

pub fn set_new_dbg_session(frame: &mut ScopeFrame<Variable>) {
    debug!("Inserting new DBG_SESSION");
    frame.insert_var(Variable::new(
        DBG_SESSION_VAR_NAME.to_string(),
        Value::DbgState(new_amtx(DbgState::default())),
        lu_source_code_item!().into(),
    ));
}

const SILENCE_VAR_NAME: &str = "SILENCE_STMT_RETURNS";
pub fn set_silence_stmt_returns(silence: bool, frame: &mut ScopeFrame<Variable>) {
    frame.insert_var(Variable::new(
        SILENCE_VAR_NAME.to_string(),
        Value::Bool(silence),
        lu_source_code_item!().into(),
    ));
}

pub fn get_silence_stmt_returns(scope: &Scope<Variable>) -> Option<bool> {
    scope
        .find_var(SILENCE_VAR_NAME)
        .map(|var| var.val.as_bool())
        .flatten()
        .cloned()
}
