#[macro_use]
extern crate derive_new;

mod command;
mod evaluate;
mod function;
mod interpreter;
mod scope;
mod variable;

pub use crate::evaluate::{EvalArg, Evaluable};
pub use crate::function::{Callable, Function};
pub use crate::interpreter::Interpreter;
pub use crate::variable::Variable;
pub use command::{Command, ARGS_VAR_NAME, IN_VAR_NAME};
pub use scope::{Scope, ScopeFrame, ScopeFrameTag, SimpleScopeFrame};
