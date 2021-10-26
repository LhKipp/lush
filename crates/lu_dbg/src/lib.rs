mod action;
mod dbg_repl;

use lu_error::LuResult;
use parking_lot::Mutex;
use std::sync::Arc;

use lu_interpreter_structs::{Scope, Variable};

use crate::dbg_repl::dbg_loop;

pub fn before_eval(stmt: &str, scope: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<()> {
    println!("Next statement: {}", stmt);
    dbg_loop(scope)
}

// pub fn after_eval<N>(
//     _: &N,
//     scope: &mut Arc<Mutex<Scope<Variable>>>,
//     result: &EvalResult,
// ) -> LuResult<()>
// where
//     N: Display,
// {
//     println!("Result: {:#?}", result);
//     dbg_loop(scope)
// }

pub fn dbg_print(msg: &str) {
    println!("{}", msg)
}
