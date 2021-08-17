use serde::{Deserialize, Serialize};
#[allow(unused)]
#[allow(dead_code)]
use std::ops::Range;

use thiserror::Error;

#[derive(Error, Debug, new, Deserialize, Serialize)]
// TODO impl display
#[error("Parse Error")]
pub struct ParseErrs {
    pub errs: Vec<ParseErr>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ParseErrKind {
    /// Catch-all
    Message(String),
}

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Parse Error")]
pub struct ParseErr {
    pub kind: ParseErrKind,
}

impl ParseErr {
    pub fn new(kind: ParseErrKind) -> Self {
        ParseErr { kind }
    }
    // fn report(&self) -> Diagnostic<()> {
    //     match self {
    //         Error::MismatchType(left, right) => Diagnostic::error()
    //             .with_code("E0308")
    //             .with_message("mismatch types")
    //             .with_labels(vec![
    //                 Label::primary((), right.range.clone()).with_message(format!(
    //                     "Expected `{}`, found: `{}`",
    //                     left.content, right.content,
    //                 )),
    //                 Label::secondary((), left.range.clone()).with_message("expected due to this"),
    //             ]),
    //         Error::MutatingImmutable(original, mutating) => Diagnostic::error()
    //             .with_code("E0384")
    //             .with_message(format!(
    //                 "cannot mutate immutable variable `{}`",
    //                 original.content,
    //             ))
    //             .with_labels(vec![
    //                 Label::secondary((), original.range.clone()).with_message(unindent::unindent(
    //                     &format!(
    //                         r#"
    //                             first assignment to `{0}`
    //                             help: make this binding mutable: `mut {0}`
    //                         "#,
    //                         original.content,
    //                     ),
    //                 )),
    //                 Label::primary((), mutating.range.clone())
    //                     .with_message("cannot assign twice to immutable variable"),
    //             ]),
    //     }
    // }
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

// /// An item in the source code to be used in the `Error` enum.
// struct SourceCodeItem {
//     range: Range<usize>,
//     content: String,
// }

// impl SourceCodeItem {
//     fn new(range: Range<usize>, content: impl Into<String>) -> SourceCodeItem {
//         let content = content.into();
//         SourceCodeItem { range, content }
//     }
// }
