// #[macro_use]
// extern crate derive_new;
// extern crate strum_macros;

mod command;
mod evaluate;
mod function;
mod interpreter;
mod scope;
mod variable;

pub use crate::evaluate::Evaluable;
pub use crate::function::Function;
pub use crate::interpreter::{CommandStorage, Interpreter};
pub use crate::variable::Variable;
pub use command::Command;
pub use scope::{Scope, ScopeFrame, ScopeFrameTag, SimpleScopeFrame};
