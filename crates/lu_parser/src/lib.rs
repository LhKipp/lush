#[macro_use]
extern crate derive_new;
extern crate strum_macros;

mod event;
pub(crate) mod generated;
pub mod grammar;
mod lexer;
mod parser;
mod serde;
mod syntax_kind;
mod token_set;

use grammar::{RootRule, Rule};
use lexer::TokenVec;

pub use crate::event::Event;
pub use crate::generated::SyntaxKind;
pub use crate::lexer::{lex, lex_tokens, Token};
pub use crate::parser::Parser;
pub(crate) use crate::token_set::TokenSet;

pub type TokenSource = TokenVec;

pub fn parse_as(input: &str, rule: &dyn Rule) -> Vec<Event> {
    let tokens = lexer::lex(input);
    let mut p = parser::Parser::new(tokens);
    rule.parse(&mut p);
    p.finish()
}

/// Parse input str as SourceFile
pub fn parse(input: &str) -> Vec<Event> {
    parse_as(input, &RootRule {})
}
