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
pub enum AstErr {
    Message(String),
    VarNotInScope(SourceCodeItem),
    StrctNotInScope(SourceCodeItem),
}

impl AstErr {
    #![allow(unused_variables)]
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            AstErr::Message(_) => todo!(),
            AstErr::VarNotInScope(_) => todo!(),
            AstErr::StrctNotInScope(_) => todo!(),
        }
    }
}

impl<S: Into<String>> From<S> for AstErr {
    fn from(s: S) -> Self {
        AstErr::Message(s.into())
    }
}

impl<T> From<AstErr> for LuResult<T> {
    fn from(e: AstErr) -> Self {
        LuResult::Err(LuErr::Ast(e))
    }
}
