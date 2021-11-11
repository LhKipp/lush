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
// #[macro_use]
// extern crate educe;
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
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Debug)]
// #[educe(Debug)]
// use derive_more::Display;
// #[display(fmt = "{}/{:?}..{:?}", content, range.start() as 32, range.end() as 32)]
pub struct SourceCodeItem {
    pub content: String,
    pub range: TextRange,
    // #[educe(Debug(ignore))]
    #[serde(skip)]
    pub sf_node_addr: usize,
}

impl SourceCodeItem {
    // TODO adapt ctor and users
    pub fn new(
        range: Range<usize>,
        content: impl Into<String>,
        sf_node_addr: impl Into<usize>,
    ) -> SourceCodeItem {
        let content = content.into();
        SourceCodeItem {
            range: TextRange::new(
                range.start.try_into().unwrap(),
                range.end.try_into().unwrap(),
            ),
            content,
            sf_node_addr: sf_node_addr.into(),
        }
    }

    pub fn is_lu_source_code_item(&self) -> bool {
        self.sf_node_addr == usize::MAX
    }

    pub fn lu_line(&self) -> usize {
        self.range.start().into()
    }

    pub fn lu_source_code_file_name(&self) -> &str {
        assert!(self.is_lu_source_code_item());
        &self.content
    }

    pub fn tmp_todo_item() -> SourceCodeItem {
        SourceCodeItem::new(999..999, "TMP_ITEM", 1337 as usize)
    }
}

/// New SourceCodeItem pointing to the file and line from the caller
#[macro_export]
macro_rules! lu_source_code_item {
    () => {
        lu_source_code_item!(0)
    };
    ($line_count_below:expr) => {{
        {
            let f_name = file!();
            log::debug!("File macro: {}", f_name);
            let line = line!();
            let line = (line as i32 + $line_count_below) as u32;
            // TODO better source code item
            lu_error::SourceCodeItem::new(
                (line as usize)..(line as usize),
                f_name.clone(),
                usize::MAX,
            )
        }
    }};
}
