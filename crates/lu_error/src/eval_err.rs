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

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum EvalErr {
    Message(String),
    VarNotFound(SourceCodeItem),
    NotConvertibleToBool(SourceCodeItem),

    SpawningExternalProcessFailed(SourceCodeItem, String),
    ExternalCmdStdinWriteErr(SourceCodeItem, String),
    ReadingStdoutFromCmdFailed(SourceCodeItem, String),
    ExternalCmdFailed(SourceCodeItem),
}

impl EvalErr {
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            // Error::MutatingImmutable(original, mutating) => Diagnostic::error()
            //     .with_code("E0384")
            //     .with_message(format!(
            //         "cannot mutate immutable variable `{}`",
            //         original.content,
            //     ))
            //     .with_labels(vec![
            //         Label::secondary((), original.range.clone()).with_message(unindent::unindent(
            //             &format!(
            //                 r#"
            //                 first assignment to `{0}`
            //                 help: make this binding mutable: `mut {0}`
            //             "#,
            //                 original.content,
            //             ),
            //         )),
            //         Label::primary((), mutating.range.clone())
            //             .with_message("cannot assign twice to immutable variable"),
            //     ]),
            EvalErr::Message(s) => Diagnostic::error().with_code("E00001").with_message(s),
            EvalErr::VarNotFound(var) => Diagnostic::error()
                .with_code("E00002")
                .with_message(format!("Variable {} not found", var.content)),
            EvalErr::NotConvertibleToBool(cond) => Diagnostic::error()
                .with_code("E00003")
                .with_message(format!("{:?}, cond {}", cond.range, cond.content)),
            EvalErr::SpawningExternalProcessFailed(cond, _) => Diagnostic::error()
                .with_code("E00004")
                .with_message(format!("{:?}, cond {}", cond.range, cond.content)),
            EvalErr::ExternalCmdStdinWriteErr(cond, _) => Diagnostic::error()
                .with_code("E00006")
                .with_message(format!("{:?}, cond {}", cond.range, cond.content)),
            EvalErr::ReadingStdoutFromCmdFailed(cond, _) => Diagnostic::error()
                .with_code("E00007")
                .with_message(format!("{:?}, cond {}", cond.range, cond.content)),
            EvalErr::ExternalCmdFailed(cond) => Diagnostic::error()
                .with_code("E00008")
                .with_message(format!("{:?}, cond {}", cond.range, cond.content)),
        }
    }
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
