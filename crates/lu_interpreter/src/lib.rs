#[macro_use]
extern crate derive_new;

mod callable;
mod evaluate;
mod interpreter;
mod scope;
mod variable;
mod typecheck;

pub use crate::evaluate::{EvalArg, Evaluable};
pub use crate::interpreter::Interpreter;
pub use crate::variable::Variable;
pub use callable::{Callable, Command, Function, RunExternalCmd, ARGS_VAR_NAME, IN_VAR_NAME};
pub use scope::{Scope, ScopeFrame, ScopeFrameTag, SimpleScopeFrame};
