extern crate derive_more;
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
mod user_def_tys;
mod value_type;
mod variable;
mod visit_arg;

pub use crate::evaluate::{EvalArg, Evaluable};
pub use crate::interpreter::{Interpreter, InterpreterCfg};
pub use crate::variable::{VarDeclNode, Variable};
pub use callable::{Callable, Command, RunExternalCmd, ARGS_VAR_NAME, ARG_VAR_NAME, IN_VAR_NAME};
pub use user_def_tys::*;

pub use evaluate::*;
pub use resolve::*;
pub use typecheck::*;

pub use scope::{Scope, ScopeFrame, ScopeFrameTag};
pub use value_type::{ValueType, ValueTypeErr};
