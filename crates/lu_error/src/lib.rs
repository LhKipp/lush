use serde::{Deserialize, Serialize};
use std::{convert::TryInto, error::Error, ops::Range};
use text_size::TextRange;

mod ast_err;
mod eval_err;
mod fs_err;
mod parse_err;
mod ty_err;
pub mod util;

#[macro_use]
extern crate derive_new;
extern crate derive_more;
extern crate strum_macros;

pub use ast_err::*;
pub use eval_err::EvalErr;
pub use fs_err::FsErr;
pub use parse_err::{ParseErr, ParseErrs};
pub use ty_err::*;

use std::result;

pub type LuResult<T> = result::Result<T, LuErr>;
pub type LuResults<T> = result::Result<T, Vec<LuErr>>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum LuErr {
    Parse(ParseErr),
    Ty(TyErr),
    FS(FsErr),
    Eval(EvalErr),
    Ast(AstErr),
    Internal(String),
    Errors(),
}

impl<E: Error> From<E> for LuErr {
    fn from(e: E) -> Self {
        LuErr::Internal(e.to_string())
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

impl From<TyErr> for LuErr {
    fn from(e: TyErr) -> Self {
        LuErr::Ty(e)
    }
}
impl From<AstErr> for LuErr {
    fn from(e: AstErr) -> Self {
        LuErr::Ast(e)
    }
}

// impl From<FsErr> for LuErr {
//     fn from(e: FsErr) -> Self {
//         LuErr::FS(e)
//     }
// }

/// An item in the source code to be used in the `Error` enum.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
// use derive_more::Display;
// #[display(fmt = "{}/{:?}..{:?}", content, range.start() as 32, range.end() as 32)]
pub struct SourceCodeItem {
    pub content: String,
    pub range: TextRange,
}

impl SourceCodeItem {
    // TODO adapt ctor and users
    pub fn new(range: Range<usize>, content: impl Into<String>) -> SourceCodeItem {
        let content = content.into();
        SourceCodeItem {
            range: TextRange::new(
                range.start.try_into().unwrap(),
                range.end.try_into().unwrap(),
            ),
            content,
        }
    }

    pub fn tmp_todo_item() -> SourceCodeItem {
        SourceCodeItem::new(999..999, "TMP_ITEM")
    }
}

/// New SourceCodeItem pointing to the file and line from the caller
#[macro_export]
macro_rules! lu_source_code_item {
    () => {{
        {
            let f_name = file!();
            let line = line!();
            // TODO better source code item
            SourceCodeItem::new(0..line as usize, f_name.clone())
        }
    }};
}
