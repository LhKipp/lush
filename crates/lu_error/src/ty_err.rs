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
    UnexpectedArg {
        arg: SourceCodeItem,
        fn_decl: SourceCodeItem,
    },
    UnsatisfiedArg {
        arg_decl: SourceCodeItem,
        cmd_stmt: SourceCodeItem,
    },
    VarExpectedToBeFunc {
        var_decl: SourceCodeItem,
        var_usage: SourceCodeItem,
    },
    VarIsNotStruct(SourceCodeItem),
    StructDoesNotHaveField {
        field_name: String,
        strct_decl: SourceCodeItem,
        usage: SourceCodeItem,
    },
}

impl TyErr {
    #![allow(unused_variables)]
    pub fn report(&self) -> Diagnostic<()> {
        match self {
            TyErr::Message(_) => todo!(),
            TyErr::TermDoesNotReturnType(_) => todo!(),
            TyErr::TypesNotEqual {
                lhs_decl,
                lhs_ty,
                rhs_decl,
                rhs_ty,
            } => todo!(),
            TyErr::UnexpectedArg { arg, fn_decl } => todo!(),
            TyErr::UnsatisfiedArg { arg_decl, cmd_stmt } => todo!(),
            TyErr::VarExpectedToBeFunc {
                var_decl,
                var_usage,
            } => todo!(),
            TyErr::VarIsNotStruct(_) => todo!(),
            TyErr::StructDoesNotHaveField {
                field_name,
                strct_decl,
                usage,
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
