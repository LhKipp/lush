mod action;
mod dbg_repl;

use lu_error::LuResult;
use lu_interpreter_structs::SyScope;

use crate::dbg_repl::dbg_loop;

pub fn before_eval(stmt: &str, scope: &mut SyScope) -> LuResult<()> {
    println!("Next statement: {}", stmt);
    dbg_loop(scope)
}

// pub fn after_eval<N>(
//     _: &N,
//     scope: &mut SyScope,
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
