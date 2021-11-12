use super::*;

use crate::{
    grammar::file_name_expr::file_name_rule,
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
};
use crate::{Token, T};

/// An expression is a source code element resembling a lu-Value
pub struct ValueExprRule;
impl Rule for ValueExprRule {
    fn name(&self) -> String {
        "expressions".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        value_expr_rule().matches(p) || p.next_non(CMT_NL_WS) == T!["("]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        // This func parses rules of value_expr_rule but with pratt parsing and math nodes
        let expr_ = expr(p);
        if expr_.is_none() {
            p.error("Expected an expression".to_string());
        }
        expr_
    }
}

/// Binding powers of operators for a Pratt parser.
///
/// See <https://www.oilshell.org/blog/2016/11/03.html>
#[rustfmt::skip]
fn next_op(p: &mut Parser) -> (u8, SyntaxKind) {
    const NOT_AN_OP: (u8, SyntaxKind)  = (0, Error);
    let next_token = p.next_non(CMT_NL_WS);
    let result = match next_token {
        // Left associative
        T![>=]                        => (5,  T![>=]),
        T![>]                         => (5,  T![>]),
        T![==]                        => (5,  T![==]),
        T![!=]                        => (5,  T![!=]),
        T![<=]                        => (5,  T![<=]),
        T![<]                         => (5,  T![<]),
        T![+]                         => (10, T![+]),
        T!["//"]                         => (11, T!["//"]),
        T![*]                         => (11, T![*]),
        T![-]                         => (10, T![-]),
        // Right associative ops
        T![=]                         => (1, T![=]),
        T![+=]                        => (1, T![+=]),
        T![-=]                        => (1, T![-=]),
        T![/=]                        => (1, T![/=]),
        T![*=]                        => (1, T![*=]),

        _                             => NOT_AN_OP
    };
    if result != NOT_AN_OP{
        p.eat_while(CMT_NL_WS);
    }
    result
}

fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_bp(p, 1)
}

fn expr_bp(p: &mut Parser, bp: u8) -> Option<CompletedMarker> {
    debug!("Parsing expr with precedence: {}", bp);
    // This is pratt parsing

    // expr_m marks the bounds of the generated expression
    let mut expr_m = lhs(p)?;

    loop {
        let (op_bp, op) = next_op(p);
        debug!(
            "Found op ({:?}) with precedence: {} (curent_precedence: {})",
            op, op_bp, bp
        );
        if op_bp < bp {
            break;
        }
        // Okay, there must be rhs now! That means, we have (at least) the following situation
        //      op <-- this is the expr we generate
        //  lhs    rhs
        // Therefore expr_m (aka lhs) must build op before building itself.
        // (Note we have 'at least' this situation, as the tree could be deeper, if we run more
        // often within the loop)
        let m = expr_m.precede(p);
        p.bump(op);

        expr_bp(p, op_bp + 1); // This will complete the rhs of the expr
                               // After we have generated the rhs in the above stmt, we now complete our expr
        expr_m = m.complete(p, MathExpr);
    }
    Some(expr_m)
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    if p.next_non(CMT_NL_WS) == T!["("] {
        p.eat_after(T!["("], CMT_NL_WS);
        let parsed_val = expr(p);
        p.expect_after(T![")"], CMT_NL_WS);
        parsed_val
    } else {
        value_expr_rule().parse(p)
    }
}

pub(crate) fn value_expr_rule() -> OrRule {
    OrRule {
        kind: Some("value expr".into()),
        rules: vec![
            Box::new(NumberExprRule {}),
            Box::new(ValuePathExprRule {}),
            Box::new(StringExprRule {}),
            Box::new(BooleanExprRule {}),
            Box::new(StrctCtorExprRule {}),
            Box::new(TableExprRule {}),
            Box::new(ArrayExprRule {}),
            Box::new(file_name_rule(true)),
            Box::new(CmdStmtRule {}),
        ],
    }
}

pub struct ArrayExprRule;
impl Rule for ArrayExprRule {
    fn name(&self) -> String {
        "Array".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["["]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.expect(T!["["]);
        // arrays are allowed to span multiple lines
        while p.eat(&[Whitespace, Newline]) || { value_expr_rule().opt(p).is_some() } {}
        p.expect(T!["]"]);
        Some(m.complete(p, ArrayExpr))
    }
}

pub struct TableExprRule;
impl Rule for TableExprRule {
    fn name(&self) -> String {
        "Table".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["["]
            // And also TableSignatureRule matches
            && p.next_non(&[Comment, Newline, Whitespace, T!["["]]) == T!["("]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after(T!["["], CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        if !p.expect_after(T!["("], CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        p.expect_after(StrctName, CMT_NL_WS);
        p.expect_after(T![")"], CMT_NL_WS);
        let array_rule = ArrayExprRule {};
        while array_rule.matches(p) {
            array_rule.parse(p);
        }
        p.expect_after(T!["]"], CMT_NL_WS);
        Some(m.complete(p, TableExpr))
    }
}

pub struct NumberExprRule;
impl Rule for NumberExprRule {
    fn name(&self) -> String {
        "Number".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == Number
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.expect(Number);
        Some(m.complete(p, NumberExpr))
    }
}

pub struct StringExprRule;
impl Rule for StringExprRule {
    fn name(&self) -> String {
        "StringExpr".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let next = p.next_non(CMT_NL_WS);
        next == SingleQuote || next == DoubleQuote
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);

        let m = p.start();
        // TODO assert is quote type
        let quote_type = p.current();
        if !p.expect(&[DoubleQuote, SingleQuote]) {
            // If quote_type is not " or ' we better don't eat any more tokens
            m.abandon(p);
            return None;
        }
        // We don't eat the content, as that would produce the string content as many
        // multiple tokens. We want the content as one token
        let str_content = p.discard_until(&[quote_type, Newline]);
        p.do_bump(Token::new(
            SyntaxKind::StringContent,
            str_content.iter().map(|t| t.len).sum(),
        ));

        if p.current() == Newline {
            p.error("Unterminated string literal".to_string());
        }

        p.eat(quote_type);
        Some(m.complete(p, StringExpr))
    }
}

pub struct BooleanExprRule;
impl Rule for BooleanExprRule {
    fn name(&self) -> String {
        "BooleanExprRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let next = p.next_non(CMT_NL_WS);
        next == TrueKeyword || next == FalseKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after([TrueKeyword, FalseKeyword], CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        Some(m.complete(p, BooleanExpr))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_as, Event};

    use super::TableExprRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/table_expr/general.yaml_test")]
    fn parse_tables(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &TableExprRule {})
    }
}
