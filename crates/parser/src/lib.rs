#[macro_use]
extern crate derive_new;

use ::serde::{Deserialize, Serialize};

mod event;
pub(crate) mod generated;
pub mod grammar;
mod lexer;
mod parser;
mod serde;
mod token_set;

use grammar::{RootRule, Rule};
use lexer::TokenVec;

pub use crate::event::Event;
pub use crate::generated::SyntaxKind;
pub use crate::lexer::{lex, lex_tokens, Token};
pub use crate::parser::Parser;
pub(crate) use crate::token_set::TokenSet;

pub type TokenSource = TokenVec;

/// `TreeSink` abstracts details of a particular syntax tree implementation.
pub trait TreeSink {
    /// Adds new token to the current branch.
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    fn error(&mut self, error: ParseError);
}

pub fn parse_from_tokens(input: &str, rule: &dyn Rule) -> Vec<Event> {
    let tokens = lexer::lex(input);
    let mut p = parser::Parser::new(tokens);
    rule.parse(&mut p);
    p.finish()
}

/// Parse given tokens into the given sink as a lu file.
pub fn parse(input: &str) -> Vec<Event> {
    parse_from_tokens(input, &RootRule {})
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParseError(pub Box<String>);
