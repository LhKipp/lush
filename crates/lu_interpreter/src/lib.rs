#[macro_use]
extern crate derive_new;

mod command;
mod evaluate;
mod function;
mod interpreter;
mod scope;
mod variable;

pub use crate::evaluate::Evaluable;
pub use crate::function::{Callable, Function};
pub use crate::interpreter::Interpreter;
pub use crate::variable::Variable;
pub use command::{Command, CMD_VAR_ARGS_NAME, CMD_VAR_IN_NAME};
pub use scope::{Scope, ScopeFrame, ScopeFrameTag, SimpleScopeFrame};
