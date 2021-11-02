use serde::{Deserialize, Serialize};

use crate::{LuErr, LuResult, SourceCodeItem};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, new)]
pub struct ParseErrs {
    pub errs: Vec<ParseErr>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ParseErr {
    /// Catch-all
    Message(String),
    MessageAt(String, SourceCodeItem),
}

impl<S: Into<String>> From<S> for ParseErr {
    fn from(s: S) -> Self {
        ParseErr::Message(s.into())
    }
}

impl<T> From<ParseErr> for LuResult<T> {
    fn from(e: ParseErr) -> Self {
        LuResult::Err(LuErr::Parse(e))
    }
}
