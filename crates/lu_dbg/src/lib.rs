use parking_lot::Mutex;
use std::{fmt::Display, sync::Arc};

use lu_interpreter_structs::{EvalResult, Scope, Variable};

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn dbg_loop(_: &Arc<Mutex<Scope<Variable>>>) {
    let mut rl = Editor::<()>::new();

    if let Ok(dbg_hist) = lu_cfg_home::dbg_history() {
        if rl.load_history(&dbg_hist).is_err() {
            println!("No previous history.");
        }
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Ok(dbg_hist) = lu_cfg_home::dbg_history() {
        if let Err(e) = rl.save_history(&dbg_hist) {
            println!("Could not save history. Error was: {}", e);
        }
    }
}

pub fn before_eval<N>(node: &N, scope: &mut Arc<Mutex<Scope<Variable>>>)
where
    N: Display,
{
    println!("Next statement: {}", node);
    dbg_loop(scope);
}

pub fn after_eval<N>(_: &N, scope: &mut Arc<Mutex<Scope<Variable>>>, result: &EvalResult)
where
    N: Display,
{
    println!("Result: {:#?}", result);
    dbg_loop(scope);
}
