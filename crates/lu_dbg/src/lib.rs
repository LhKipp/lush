mod action;
use action::ALL_DBG_ACTIONS_WITH_AFTER_EVAL;
use lu_stdx::AMtx;
mod dbg_repl;

use std::rc::Rc;

use lu_error::LuResult;
use lu_interpreter_structs::{dbg_state::DbgState, *};
use lu_syntax::{AstId, Parse};

use crate::{action::DbgNextAction, dbg_repl::dbg_loop};

#[derive(Debug)]
pub enum DbgIntervention {
    ContinueAsIfStmtRet(Parse),
    ContinueAsIfStmtRetsNil,
}

fn get_dbg_session(scope: SyScope) -> Option<AMtx<DbgState>> {
    let l_scope = scope.lock();
    lu_interpreter_structs::get_dbg_session(&l_scope).cloned()
}

pub fn before_eval(
    stmt: &str,
    stmt_id: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    if let Some(dbg_state) = get_dbg_session(scope.clone()) {
        // To provide "concurrent" debugging of parallel programs, we lock early here
        let dbg_state_l = &mut dbg_state.lock();

        if !DbgNextAction::allows_dbg_intervention(&dbg_state_l) {
            return Ok(None);
        }
        println!("Next statement: {}", stmt);
        dbg_loop(dbg_state_l, stmt_id, scope)
    } else {
        Ok(None)
    }
}

pub fn after_eval(stmt_id: &AstId, scope: &mut SyScope) {
    if let Some(dbg_state) = get_dbg_session(scope.clone()) {
        for action in &*ALL_DBG_ACTIONS_WITH_AFTER_EVAL {
            // To provide "concurrent" debugging of parallel programs, we lock early here
            let dbg_state_l = &mut dbg_state.lock();

            action.exec_after_eval(stmt_id, dbg_state_l, scope);
        }
    }

    // TODO
}

pub fn warn_unpure_cmd_call(
    cmd: &Rc<dyn Command>,
    ast_id: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    if let Some(dbg_state) = get_dbg_session(scope.clone()) {
        let dbg_state_l = &mut dbg_state.lock();
        // TODO required flags are also necessary
        let cmd_id_str = cmd.name();
        dbg_print(&format!(
            r#"Warning: Running {cmd_name} might have side effects.
Type "skip [Value]" to skip running {cmd_name} and continue as if the the cmd returned [Value]
Type "next" or "step" to run the cmd
Type "help" for further help"#,
            cmd_name = cmd_id_str
        ));
        dbg_loop(dbg_state_l, ast_id, scope)
    } else {
        Ok(None)
    }
}

pub fn dbg_print(msg: &str) {
    println!("{}", msg)
}
