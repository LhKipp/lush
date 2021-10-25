#![allow(unused_variables)]
use parking_lot::Mutex;
use std::{fmt::Display, sync::Arc};

use lu_interpreter_structs::{EvalResult, Scope, Variable};

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn dbg_loop() {
    let mut rl = Editor::<()>::new();

    if let Ok(cfg_home) = lu_cfg_home::cfg_home() {
        if rl.load_history(&cfg_home).is_err() {
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
    rl.save_history("history.txt").unwrap();
}

pub fn before_eval<N>(node: &N, scope: &mut Arc<Mutex<Scope<Variable>>>)
where
    N: Display,
{
    println!("Next statement: {}", node);
}

pub fn after_eval<N>(node: &N, scope: &mut Arc<Mutex<Scope<Variable>>>, result: &EvalResult)
where
    N: Display,
{
    println!("Result: {:#?}", result);
}
