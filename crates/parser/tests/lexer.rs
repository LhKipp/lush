use parser::{lex_tokens, Token};
use {conformance, serde_yaml};

#[conformance::tests(exact, serde=serde_yaml, file="test_data/lexer.yaml")]
fn lex(s: &str) -> Vec<Token> {
    lex_tokens(s)
}
