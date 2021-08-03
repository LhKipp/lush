#[allow(unused_imports)]
use super::*;

use crate::T;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, ValueExpr, *},
    TokenSet,
};

/// Binding powers of operators for a Pratt parser.
///
/// See <https://www.oilshell.org/blog/2016/11/03.html>
#[rustfmt::skip]
fn next_op(p: &mut Parser) -> (u8, SyntaxKind) {
    const NOT_AN_OP: (u8, SyntaxKind)  = (0, Error);
    let next_token = p.next_non(CMT_NL_WS);
    let result = match next_token {
        T![>=]                        => (5,  T![>=]),
        T![>]                         => (5,  T![>]),
        T![==]                        => (5,  T![==]),
        T![!=]                        => (5,  T![!=]),
        T![<=]                        => (5,  T![<=]),
        T![<]                         => (5,  T![<]),
        T![+]                         => (10, T![+]),
        T![/]                         => (11, T![/]),
        T![*]                         => (11, T![*]),
        T![-]                         => (10, T![-]),
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
    value_expr_rule().opt(p)
}

/// A expression is a combination of values by operators
pub(crate) struct ExpressionsRule;
impl Rule for ExpressionsRule {
    fn name(&self) -> String {
        "expressions".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        value_expr_rule().matches(p)
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let expr_ = expr(p);
        if expr_.is_none() {
            p.error("Expected an expression");
        }
        expr_
    }
}

pub(crate) fn value_expr_rule() -> OrRule {
    OrRule {
        kind: Some("value expr".into()),
        rules: vec![
            Box::new(NumberRule {}),
            Box::new(ValuePathRule {}),
            Box::new(StringRule {}),
            Box::new(BareWord),
            Box::new(table_or_array_rule()),
        ],
    }
}

pub(crate) fn table_or_array_rule() -> OrRule {
    OrRule {
        kind: None,
        rules: vec![Box::new(TableRule {}), Box::new(ArrayRule {})],
    }
}

pub struct ArrayRule;
impl Rule for ArrayRule {
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
        Some(m.complete(p, Array))
    }
}

pub struct TableRule;
impl Rule for TableRule {
    fn name(&self) -> String {
        "Table".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["["]
            && p.next_non(&[Comment, Newline, Whitespace, T!["["]]) == T!["("]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.eat(T!["["]);
        // arrays are allowed to span multiple lines
        while p.eat(&[Whitespace, Newline]) || { value_expr_rule().opt(p).is_some() } {}
        p.expect(T!["]"]);
        Some(m.complete(p, Table))
    }
}

pub struct NumberRule;
impl Rule for NumberRule {
    fn name(&self) -> String {
        "Number".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == Number
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_while(CMT_NL_WS);
        p.expect(Number);
        Some(m.complete(p, Number))
    }
}

pub struct StringRule;
impl Rule for StringRule {
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
        p.eat(quote_type);
        p.eat_until(&[quote_type, Newline]);

        if p.current() == Newline {
            p.error("Unterminated string literal");
        }

        let quote_type = match quote_type {
            DoubleQuote => {
                p.eat(DoubleQuote);
                DoubleQuotedString
            }
            SingleQuote => {
                p.eat(SingleQuote);
                SingleQuotedString
            }
            _ => unreachable!("quote type either double or single"),
        };

        Some(m.complete(p, quote_type))
    }
}

struct ValuePathRule;
impl Rule for ValuePathRule {
    fn name(&self) -> String {
        "ValuePath".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T![$]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.eat(Dollar);
        loop {
            if !p.expect(BareWord) {
                break;
            }
            if !p.at(Point) {
                break;
            }
        }
        Some(m.complete(p, ValuePath))
    }
}
