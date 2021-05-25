use logos::Logos;

use crate::SyntaxKind;

#[derive(Debug, PartialEq, new)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: i32,
}

pub fn lex<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    let lex = SyntaxKind::lexer(input).spanned();
    return lex.map(|(kind, span)| Token::new(kind, span.len() as i32));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SyntaxKind;
    #[test]
    fn test_lex() {
        let tokens = lex("fn | |");
        let tokens = tokens.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::new(SyntaxKind::Fn, 2));
        assert_eq!(tokens[1], Token::new(SyntaxKind::Whitespace, 1));
        assert_eq!(tokens[2], Token::new(SyntaxKind::Pipe, 1));
        assert_eq!(tokens[3], Token::new(SyntaxKind::Whitespace, 1));
        assert_eq!(tokens[4], Token::new(SyntaxKind::Pipe, 1));
    }
}
