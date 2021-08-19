use serde::{Deserialize, Serialize};
use std::ops::Range;

mod eval_err;
mod fs_err;
mod parse_err;

#[macro_use]
extern crate derive_new;
extern crate strum_macros;

pub use eval_err::EvalErr;
pub use fs_err::FsErr;
pub use parse_err::{ParseErr, ParseErrKind, ParseErrs};

use std::result;

pub type LuResult<T> = result::Result<T, LuErr>;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LuErr {
    Parse(ParseErr),
    ParseErrs(ParseErrs),
    Eval(EvalErr),
    FS(FsErr),
    Internal(String),
}

impl<S: Into<String>> From<S> for LuErr {
    fn from(s: S) -> Self {
        LuErr::Internal(s.into())
    }
}

impl From<ParseErr> for LuErr {
    fn from(e: ParseErr) -> Self {
        LuErr::Parse(e)
    }
}

impl From<EvalErr> for LuErr {
    fn from(e: EvalErr) -> Self {
        LuErr::Eval(e)
    }
}

impl From<FsErr> for LuErr {
    fn from(e: FsErr) -> Self {
        LuErr::FS(e)
    }
}

/// An item in the source code to be used in the `Error` enum.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SourceCodeItem {
    range: Range<usize>,
    content: String,
}

impl SourceCodeItem {
    pub fn new(range: Range<usize>, content: impl Into<String>) -> SourceCodeItem {
        let content = content.into();
        SourceCodeItem { range, content }
    }
}
