use serde::{Deserialize, Serialize};

mod fs_err;
mod parse_err;

#[macro_use]
extern crate derive_new;
extern crate strum_macros;

pub use fs_err::FsErr;
pub use parse_err::{ParseErr, ParseErrKind, ParseErrs};

use std::result;
use thiserror::Error;

pub type LuResult<T> = result::Result<T, LuErr>;

#[derive(Error, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[error("{0}")]
pub enum LuErr {
    Parse(#[from] ParseErr),
    ParseErrs(#[from] ParseErrs),
    // Eval(#[from] EvalErr),
    FS(#[from] FsErr),
    Internal(String),
}

impl<S: Into<String>> From<S> for LuErr {
    fn from(s: S) -> Self {
        LuErr::Internal(s.into())
    }
}
