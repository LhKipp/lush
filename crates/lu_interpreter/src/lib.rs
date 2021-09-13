#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate educe;

mod callable;
mod evaluate;
mod interpreter;
mod scope;
pub mod typecheck;
mod variable;

pub use crate::evaluate::{EvalArg, Evaluable};
pub use crate::interpreter::Interpreter;
pub use crate::variable::Variable;
pub use callable::{
    ArgSignature, Callable, Command, FlagSignature, Function, RunExternalCmd, Signature,
    VarArgSignature, ARGS_VAR_NAME, ARG_VAR_NAME, IN_VAR_NAME,
};
pub use scope::{Scope, ScopeFrame, ScopeFrameTag, SimpleScopeFrame};
