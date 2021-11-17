#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{self, ColorArg};
use std::ops::Range;

use thiserror::Error;

use crate::{LuErr, LuResult, SourceCodeItem};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum EvalErr {
    Message(String),
    VarNotFound(SourceCodeItem),
    NotConvertibleToBool(SourceCodeItem),

    SpawningExternalProcessFailed(SourceCodeItem, String),
    ExternalCmdStdinWriteErr(SourceCodeItem, String),
    ExternalCmdStdoutReadErr(SourceCodeItem, String),
    ExternalCmdFailed(SourceCodeItem),

    // Pseudo err to conveniently return execution. Does not print anything
    BadCast {
        cast_math_expr: SourceCodeItem,
        expected_ty: String,
        value_item: SourceCodeItem,
        value_ty: String,
    },
    DbgAbort,
}

impl<S: Into<String>> From<S> for EvalErr {
    fn from(s: S) -> Self {
        EvalErr::Message(s.into())
    }
}

impl<T> From<EvalErr> for LuResult<T> {
    fn from(e: EvalErr) -> Self {
        LuResult::Err(LuErr::Eval(e))
    }
}
