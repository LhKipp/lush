mod command;
mod function;
mod run_external_cmd;

pub use command::{Command, ARGS_VAR_NAME, ARG_VAR_NAME, IN_VAR_NAME};
use derive_more::From;
use enum_as_inner::EnumAsInner;
pub use function::*;
use lu_error::SourceCodeItem;
pub use run_external_cmd::RunExternalCmd;

use crate::{EvalArg, Evaluator, Function, Signature};

#[derive(Clone, Debug, EnumAsInner, From)]
pub enum Callable {
    Func(Function),
    InternalCmd(Box<dyn Command>),
    ExternalCmd(RunExternalCmd),
}

impl Command for Callable {
    fn do_run(&self, _: &[EvalArg], state: &mut Evaluator) -> lu_error::LuResult<lu_value::Value> {
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

    fn signature(&self) -> &Signature {
        match self {
            Callable::Func(f) => f.signature(),
            Callable::InternalCmd(cmd) => cmd.signature(),
            Callable::ExternalCmd(cmd) => cmd.signature(),
        }
    }

    fn signature_item(&self) -> SourceCodeItem {
        match self {
            Callable::Func(f) => f.signature_item(),
            Callable::InternalCmd(cmd) => cmd.signature_item(),
            Callable::ExternalCmd(cmd) => cmd.signature_item(),
        }
    }
}
