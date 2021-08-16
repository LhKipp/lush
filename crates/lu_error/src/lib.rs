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

#[derive(Error, Debug)]
#[error("Parse Error")]
pub enum ParseErr {}

#[derive(Error, Debug)]
#[error("Eval Error")]
pub enum EvalErr {}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum LuErr {
    Parsing(#[from] ParseErr),
    Eval(#[from] EvalErr),
    FS(#[from] FsErr),
}
