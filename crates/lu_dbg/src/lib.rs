mod action;
mod dbg_repl;

use std::rc::Rc;

use lu_error::LuResult;
use lu_interpreter_structs::*;

use crate::dbg_repl::dbg_loop;

pub enum DbgIntervention {
    ContinueAsIfStmtRet(Value),
}

pub fn before_eval(stmt: &str, scope: &mut SyScope) -> LuResult<Option<DbgIntervention>> {
    println!("Next statement: {}", stmt);
    dbg_loop(scope)
}

pub fn warn_unpure_cmd_call(
    cmd: &Rc<dyn Command>,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    // TODO required flags are also necessary
    let cmd_id_str = cmd.name();
    dbg_print(&format!(
        r#"Warning: Running {cmd_name} might have side effects.
Type "skip <Value>" to skip running {cmd_name} and continue as if the the cmd returned <Value>
Type "next" or "step" to run the cmd
Type "help" for further help"#,
        cmd_name = cmd_id_str
    ));
    dbg_loop(scope)
}

pub fn dbg_print(msg: &str) {
    println!("{}", msg)
}
