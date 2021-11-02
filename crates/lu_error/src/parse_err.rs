use serde::{Deserialize, Serialize};
use text_size::TextSize;

use crate::{LuErr, LuResult, SourceCodeItem};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, new)]
pub struct ParseErrs {
    pub errs: Vec<ParseErr>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ParseErr {
    /// Catch-all
    MessageAt(String, TextSize),
    MessageAtItem(String, SourceCodeItem),
}

impl<T> From<ParseErr> for LuResult<T> {
    fn from(e: ParseErr) -> Self {
        LuResult::Err(LuErr::Parse(e))
    }
}
