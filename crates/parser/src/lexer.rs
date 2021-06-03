use std::ops::{Index, Range};

use logos::Logos;

use crate::{SyntaxKind, SyntaxKind::*};

#[derive(Debug, PartialEq, new, Clone, Copy)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: i32,
}

pub const T_EOF: Token = Token::eof();
impl Token {
    pub const fn eof() -> Token {
        Token { kind: Eof, len: 0 }
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
            end: n.end,
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
        self.tokens[self.cur_elem..].iter()
    }
    pub fn bump(&mut self) {
        self.cur_elem = self.cur_elem + 1;
    }
}

pub fn lex<'a>(input: &'a str) -> TokenVec {
    let lex = SyntaxKind::lexer(input).spanned();
    TokenVec::new(
        lex.map(|(kind, span)| Token::new(kind, span.len() as i32))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SyntaxKind;
    #[test]
    fn test_lex() {
        let tokens = lex("fn | |");
        assert_eq!(tokens[0], Token::new(SyntaxKind::Fn, 2));
        assert_eq!(tokens[1], Token::new(SyntaxKind::Whitespace, 1));
        assert_eq!(tokens[2], Token::new(SyntaxKind::Pipe, 1));
        assert_eq!(tokens[3], Token::new(SyntaxKind::Whitespace, 1));
        assert_eq!(tokens[4], Token::new(SyntaxKind::Pipe, 1));
        assert_eq!(tokens[5], Token::eof());
    }
}
