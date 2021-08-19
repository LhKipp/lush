use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Error, Debug, new, Deserialize, Serialize, PartialEq, Eq)]
// TODO impl display
#[error("Parse Error")]
pub struct ParseErrs {
    pub errs: Vec<ParseErr>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ParseErrKind {
    /// Catch-all
    Message(String),
}

#[derive(Error, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[error("Parse Error")]
pub struct ParseErr {
    pub kind: ParseErrKind,
}

impl ParseErr {
    pub fn new(kind: ParseErrKind) -> Self {
        ParseErr { kind }
    }
}

impl<S: Into<String>> From<S> for ParseErr {
    fn from(s: S) -> Self {
        ParseErr::new(ParseErrKind::Message(s.into()))
    }
}
impl From<ParseErrKind> for ParseErr {
    fn from(e: ParseErrKind) -> Self {
        ParseErr::new(e)
    }
}
