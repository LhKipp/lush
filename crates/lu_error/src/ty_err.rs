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
        var_usage: SourceCodeItem,
    },
    ItemExpectedToBeFunc(SourceCodeItem),
    ItemExpectedToBeStruct(SourceCodeItem),
    ItemExpectedToBeArray(SourceCodeItem),
    ItemExpectedToBeOptional(SourceCodeItem),
    StructDoesNotHaveField {
        field_name: String,
        strct_decl: SourceCodeItem,
        usage: SourceCodeItem,
    },
    FlagWithoutArgument(SourceCodeItem),
    PassingOfNotDeclaredFlag(SourceCodeItem),
    NotPassedRequiredFlag {
        flag_decl: SourceCodeItem,
        cmd_stmt: SourceCodeItem,
    },
    TableRowToManyCol {
        row: SourceCodeItem,
    },
    TableRowToFewCol {
        row: SourceCodeItem,
    },
    ExpectedStmtToBeInferred {
        stmt: SourceCodeItem,
    },
    ExpectedStmtToReturnAnArrayOfStrcts {
        found_ty: String,
        stmt_with_wrong_ret: SourceCodeItem,
    },
    SelectArgMustBeBareWordOrString {
        arg: SourceCodeItem,
    },
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
