mod parse_err;

#[macro_use]
extern crate derive_new;
extern crate strum_macros;

pub use parse_err::{ParseErr, ParseErrKind, ParseErrs};

use std::{io, path::PathBuf, result};
use thiserror::Error;

pub type LuResult<T> = result::Result<T, LuErr>;

/// Helper struct for FsErr
#[derive(Error, Debug)]
#[error("{source}")]
pub struct InnerIoErr {
    #[from]
    source: io::Error,
}

#[derive(Error, Debug)]
#[error("IO-Error using path {path}:\n{source}")]
pub struct FsErr {
    pub path: PathBuf,
    pub source: InnerIoErr,
}

#[derive(Error, Debug, Clone)]
#[error("Eval Error")]
pub enum EvalErr {}

#[derive(Error, Debug)]
#[error("{0}")]
pub enum LuErr {
    Parse(#[from] ParseErr),
    ParseErrs(#[from] ParseErrs),
    Eval(#[from] EvalErr),
    FS(#[from] FsErr),
    Internal(String),
}

impl<S: Into<String>> From<S> for LuErr {
    fn from(s: S) -> Self {
        LuErr::Internal(s.into())
    }
}
