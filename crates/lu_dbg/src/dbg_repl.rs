use crate::{action::*, DbgIntervention};
use lu_error::{EvalErr, LuResult};
use lu_interpreter_structs::dbg_state::DbgState;
use lu_interpreter_structs::SyScope;
use lu_syntax::AstId;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use vec_box::vec_box;

pub fn dbg_loop(
    _: &mut DbgState,
    _: AstId,
    scope: &mut SyScope,
) -> LuResult<Option<DbgIntervention>> {
    let mut rl = Editor::<()>::new();

    if let Ok(dbg_hist) = lu_cfg_home::dbg_history() {
        if rl.load_history(&dbg_hist).is_err() {
            println!("No previous history.");
        }
    }

    let cmds: Vec<Box<dyn DbgAction>> = vec_box![
        DbgStepAction {},
        DbgNextAction {},
        DbgSkipAction {},
        DbgPrintAction {},
        DbgScopeAction {}
    ];
    let ret_val = loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                rl.add_history_entry(line);
                // Handle help a little special.
                if line.starts_with("help") {
                    print_help(&cmds);
                    continue;
                }

                let mut cmd_exec_action = None;
                for cmd in &cmds {
                    if cmd.matches(&line) {
                        cmd_exec_action = Some(cmd.exec(&line, scope));
                        break;
                    }
                }

                match cmd_exec_action {
                    Some(DbgActionResult::StopDbgLoop) => break Ok(()),
                    None => {
                        print_help(&cmds);
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

fn print_help(cmds: &Vec<Box<dyn DbgAction>>) {
    println!(
        r#"Commands: 
  help - show this help"#
    );
    for cmd in cmds {
        let args = if !cmd.args().is_empty() {
            format!(", {}", cmd.args().join(" "))
        } else {
            "".to_string()
        };
        println!(
            "  {}, {}{} - {}",
            cmd.long_name(),
            cmd.short_name(),
            args,
            cmd.description()
        );
    }
}
