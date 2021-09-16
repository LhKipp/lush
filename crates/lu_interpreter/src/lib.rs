#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate educe;

mod callable;
mod evaluate;
mod interpreter;
mod resolve;
mod scope;
mod typecheck;
mod value_type;
mod variable;
mod visit_arg;

pub use crate::evaluate::{EvalArg, Evaluable};
pub use crate::interpreter::Interpreter;
pub use crate::variable::{VarDeclNode, Variable};
pub use callable::{
    ArgSignature, Callable, Command, FlagSignature, Function, InArgSignature, RetArgSignature,
    RunExternalCmd, Signature, VarArgSignature, ARGS_VAR_NAME, ARG_VAR_NAME, IN_VAR_NAME,
};

pub use evaluate::*;
pub use resolve::*;
pub use typecheck::*;

pub use scope::{Scope, ScopeFrame, ScopeFrameTag};
pub use value_type::{ValueType, ValueTypeErr};
