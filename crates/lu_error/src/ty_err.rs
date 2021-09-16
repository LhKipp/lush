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
pub enum TyErr {
    Message(String),
    TermDoesNotReturnType(SourceCodeItem),
    TypesNotEqual {
        lhs_decl: Option<SourceCodeItem>,
        lhs_ty: String,
        rhs_decl: Option<SourceCodeItem>,
        rhs_ty: String,
    },
}

impl TyErr {
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            TyErr::Message(_) => todo!(),
            TyErr::TermDoesNotReturnType(_) => todo!(),
            #[allow(unused_variables)]
            TyErr::TypesNotEqual {
                lhs_decl,
                lhs_ty,
                rhs_decl,
                rhs_ty,
            } => todo!(),
        }
    }
}

impl<S: Into<String>> From<S> for TyErr {
    fn from(s: S) -> Self {
        TyErr::Message(s.into())
    }
}

impl<T> From<TyErr> for LuResult<T> {
    fn from(e: TyErr) -> Self {
        LuResult::Err(LuErr::Ty(e))
    }
}
