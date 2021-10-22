use derive_more::From;
use lu_error::LuErr;

use crate::Value;

#[derive(Debug, From, Clone)]
pub enum RetValOrErr {
    RetVal(Value),
    Err(LuErr),
}

pub type EvalResult = Result<Value, RetValOrErr>;

impl From<RetValOrErr> for EvalResult {
    fn from(e: RetValOrErr) -> Self {
        Err(e)
    }
}
