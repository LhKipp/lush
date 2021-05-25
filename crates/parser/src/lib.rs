#[macro_use]
extern crate derive_new;

mod event;
pub(crate) mod generated;
mod lexer;
mod parser;

pub use crate::generated::SyntaxKind;
pub use crate::lexer::{lex, Token};

/// `TokenSource` abstracts the source of the tokens parser operates on.
///
/// Hopefully this will allow us to treat text and token trees in the same way!
pub trait TokenSource {
    fn current(&self) -> Token;

    /// Lookahead n token
    fn lookahead_nth(&self, n: usize) -> Token;

    /// bump cursor to next token
    fn bump(&mut self);

    /// Is the current token a specified keyword?
    fn is_keyword(&self, kw: &str) -> bool;
}

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

fn parse_from_tokens<F>(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink, f: F)
where
    F: FnOnce(&mut parser::Parser),
{
    let mut p = parser::Parser::new(token_source);
    f(&mut p);
    let events = p.finish();
    event::process(tree_sink, events);
}

// /// Parse given tokens into the given sink as a rust file.
// pub fn parse(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink) {
//     parse_from_tokens(token_source, tree_sink, grammar::root);
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(pub Box<String>);

/// A bit-set of `SyntaxKind`s
#[derive(Clone, Copy)]
pub(crate) struct TokenSet(u128);

impl TokenSet {
    pub(crate) const EMPTY: TokenSet = TokenSet(0);

    pub(crate) const fn new(kinds: &[SyntaxKind]) -> TokenSet {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1
        }
        TokenSet(res)
    }

    pub(crate) const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 | other.0)
    }

    pub(crate) const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & mask(kind) != 0
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}

// #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
// pub enum FragmentKind {
//     Path,
//     Expr,
//     Statement,
//     StatementOptionalSemi,
//     Type,
//     Pattern,
//     Item,
//     Block,
//     Visibility,
//     MetaItem,

//     // These kinds are used when parsing the result of expansion
//     // FIXME: use separate fragment kinds for macro inputs and outputs?
//     Items,
//     Statements,

//     Attr,
// }

// pub fn parse_fragment(
//     token_source: &mut dyn TokenSource,
//     tree_sink: &mut dyn TreeSink,
//     fragment_kind: FragmentKind,
// ) {
//     let parser: fn(&'_ mut parser::Parser) = match fragment_kind {
//         FragmentKind::Path => grammar::fragments::path,
//         FragmentKind::Expr => grammar::fragments::expr,
//         FragmentKind::Type => grammar::fragments::type_,
//         FragmentKind::Pattern => grammar::fragments::pattern_single,
//         FragmentKind::Item => grammar::fragments::item,
//         FragmentKind::Block => grammar::fragments::block_expr,
//         FragmentKind::Visibility => grammar::fragments::opt_visibility,
//         FragmentKind::MetaItem => grammar::fragments::meta_item,
//         FragmentKind::Statement => grammar::fragments::stmt,
//         FragmentKind::StatementOptionalSemi => grammar::fragments::stmt_optional_semi,
//         FragmentKind::Items => grammar::fragments::macro_items,
//         FragmentKind::Statements => grammar::fragments::macro_stmts,
//         FragmentKind::Attr => grammar::fragments::attr,
//     };
//     parse_from_tokens(token_source, tree_sink, parser)
// }
