mod action;
mod dbg_repl;

use std::rc::Rc;

use lu_error::LuResult;
use lu_interpreter_structs::*;
use lu_syntax::{AstId, Parse};

use crate::dbg_repl::dbg_loop;

pub enum DbgIntervention {
    ContinueAsIfStmtRet(Parse),
}

pub fn before_eval(
    stmt: &str,
    stmt_id: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    if let Some(dbg_state) = get_dbg_session(&scope.clone().lock()).cloned() {
        // To provide "concurrent" debugging of parallel programs, we lock early here
        let dbg_state_l = &mut dbg_state.lock();
        println!("Next statement: {}", stmt);
        dbg_loop(dbg_state_l, stmt_id, scope)
    } else {
        Ok(None)
    }
}

pub fn after_eval(_: &str, _: &AstId, _: &mut SyScope) {
    // TODO
}

pub fn warn_unpure_cmd_call(
    cmd: &Rc<dyn Command>,
    ast_id: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    if let Some(dbg_state) = get_dbg_session(&scope.clone().lock()).cloned() {
        let dbg_state_l = &mut dbg_state.lock();
        // TODO required flags are also necessary
        let cmd_id_str = cmd.name();
        dbg_print(&format!(
            r#"Warning: Running {cmd_name} might have side effects.
Type "skip <Value>" to skip running {cmd_name} and continue as if the the cmd returned <Value>
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
