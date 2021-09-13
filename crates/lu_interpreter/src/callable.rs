mod command;
mod function;
mod run_external_cmd;

pub use command::{Command, ARGS_VAR_NAME, ARG_VAR_NAME, IN_VAR_NAME};
pub use function::*;
pub use run_external_cmd::RunExternalCmd;

use crate::EvalArg;

#[derive(Clone, Debug)]
pub enum Callable {
    Func(Function),
    InternalCmd(Box<dyn Command>),
    ExternalCmd(RunExternalCmd),
}

impl Callable {}

impl Command for Callable {
    fn do_run(
        &self,
        _: &[EvalArg],
        state: &mut crate::Interpreter,
    ) -> lu_error::LuResult<lu_value::Value> {
        match self {
            Callable::Func(f) => f.run(state),
            Callable::InternalCmd(cmd) => cmd.run(state),
            Callable::ExternalCmd(cmd) => cmd.run(state),
        }
    }

    fn name(&self) -> &str {
        match self {
            Callable::Func(f) => f.name(),
            Callable::InternalCmd(cmd) => cmd.name(),
            Callable::ExternalCmd(cmd) => cmd.name(),
        }
    }
}

impl From<Box<dyn Command>> for Callable {
    fn from(cmd: Box<dyn Command>) -> Self {
        Callable::InternalCmd(cmd)
    }
}

impl From<Function> for Callable {
    fn from(func: Function) -> Self {
        Callable::Func(func)
    }
}
