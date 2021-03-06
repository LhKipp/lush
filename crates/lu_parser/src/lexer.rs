use lazy_static::lazy_static;
use std::{
    convert::TryInto,
    ops::{Index, Range, RangeFrom},
};

use logos::Logos;
use text_size::TextSize;

use crate::{SyntaxKind, SyntaxKind::*};

#[derive(Debug, PartialEq, new, Clone, Copy)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: TextSize,
}

lazy_static! {
    static ref T_EOF: Token = Token::eof();
}
impl Token {
    pub fn eof() -> Token {
        Token {
            kind: Eof,
            len: 0u32.into(),
        }
    }
}

pub struct TokenVec {
    tokens: Vec<Token>,
    cur_elem: usize,
}

impl Index<usize> for TokenVec {
    type Output = Token;

    fn index(&self, n: usize) -> &Self::Output {
        if n + self.cur_elem >= self.tokens.len() {
            &T_EOF
        } else {
            &self.tokens[self.cur_elem + n]
        }
    }
}

impl Index<Range<usize>> for TokenVec {
    type Output = [Token];

    fn index(&self, n: Range<usize>) -> &Self::Output {
        let r = Range {
            start: self.cur_elem + n.start,
            end: self.cur_elem + n.end,
        };
        &self.tokens[r]
    }
}

impl Index<RangeFrom<usize>> for TokenVec {
    type Output = [Token];

    fn index(&self, n: RangeFrom<usize>) -> &Self::Output {
        let r = RangeFrom {
            start: self.cur_elem + n.start,
        };
        &self.tokens[r]
    }
}

impl TokenVec {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenVec {
            tokens,
            cur_elem: 0,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self[0..].iter()
    }
    pub fn bump(&mut self) {
        self.cur_elem = self.cur_elem + 1;
    }
    pub fn take_and_advance(&mut self) -> Token {
        let current = self[0];
        self.bump();
        return current;
    }
}

pub fn lex_tokens(input: &str) -> Vec<Token> {
    let lex = SyntaxKind::lexer(input).spanned();
    lex.map(|(kind, span)| Token::new(kind, span.len().try_into().unwrap()))
        .collect()
}

pub fn lex(input: &str) -> TokenVec {
    TokenVec::new(lex_tokens(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/lexer.yaml")]
    fn lex(s: &str) -> Vec<Token> {
        lex_tokens(s)
    }
}
