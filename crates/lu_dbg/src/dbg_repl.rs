use crate::action::ALL_DBG_ACTIONS;
use crate::{action::*, DbgIntervention};
use lu_error::{EvalErr, LuResult};
use lu_interpreter_structs::dbg_state::DbgState;
use lu_interpreter_structs::SyScope;
use lu_syntax::AstId;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn dbg_loop(
    dbg_state: &mut DbgState,
    stmt_id: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    let mut rl = Editor::<()>::new();

    if let Ok(dbg_hist) = lu_cfg_home::dbg_history() {
        if rl.load_history(&dbg_hist).is_err() {
            println!("No previous history.");
        }
    }

    let ret_val = loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);

                let mut cmd_exec_action = None;
                for cmd in &*ALL_DBG_ACTIONS {
                    if cmd.matches(&line) {
                        cmd_exec_action = Some(cmd.exec(&line, &stmt_id, dbg_state, scope));
                        break;
                    }
                }

                match cmd_exec_action {
                    Some(DbgActionResult::StopDbgLoop) => break Ok(()),
                    None => {
                        DbgHelpAction {}.exec(&line, &stmt_id, dbg_state, scope);
                    }
                    _ => {} // keep going
                }
            }
            Err(ReadlineError::Interrupted) => {
                // ctrl-c
                break EvalErr::DbgAbort.into();
            }
            Err(ReadlineError::Eof) => {
                // ctrl-d
                break EvalErr::DbgAbort.into();
            }
            Err(err) => {
                println!("Unexpected err: {}.", err)
            }
        }
    };

    if let Ok(dbg_hist) = lu_cfg_home::dbg_history() {
        if let Err(e) = rl.save_history(&dbg_hist) {
            println!("Could not save history. Error was: {}", e);
        }
    }

    // TODO return proper DbgIntervention
    ret_val.map(|_| None)
}
