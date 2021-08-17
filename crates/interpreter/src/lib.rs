// #[macro_use]
// extern crate derive_new;
// extern crate strum_macros;

mod command;
mod evaluate;
mod interpreter;
mod scope;

pub use crate::evaluate::Evaluable;
pub use crate::interpreter::{CommandStorage, Interpreter};
pub use command::Command;
pub use scope::{Scope, ScopeFrame, SimpleScopeFrame};
