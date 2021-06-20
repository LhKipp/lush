#[allow(unused_imports)]
use super::*;

use crate::T;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, ValueExpr, *},
    TokenSet,
};

pub(crate) struct ExpressionsRule;
impl Rule for ExpressionsRule {
    fn name(&self) -> String {
        "expressions".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        value_expr_rule().matches(p)
    }

    fn parse_rule(&self, p: &mut Parser) {
        value_expr_rule().parse_rule(p)
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

    fn parse_rule(&self, p: &mut Parser) {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.expect(T!["["]);
        // arrays are allowed to span multiple lines
        while p.eat(&[Whitespace, Newline]) || { value_expr_rule().opt(p) } {}
        p.expect(T!["]"]);
        m.complete(p, Array);
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

    fn parse_rule(&self, p: &mut Parser) {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.eat(T!["["]);
        // arrays are allowed to span multiple lines
        while p.eat(&[Whitespace, Newline]) || { value_expr_rule().opt(p) } {}
        p.expect(T!["]"]);
        m.complete(p, Table);
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

    fn parse_rule(&self, p: &mut Parser) {
        p.eat(Number);
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

    fn parse_rule(&self, p: &mut Parser) {
        p.eat_while(CMT_NL_WS);

        let m = p.start();
        let quote_type = p.current();
        p.eat(quote_type);
        p.eat_until(&[quote_type, Newline]);

        if p.current() == Newline {
            p.error("Unterminated string literal");
        }

        match quote_type {
            DoubleQuote => m.complete(p, DoubleQuotedString),
            SingleQuote => m.complete(p, SingleQuotedString),
            _ => unreachable!("quote type either double or single"),
        };
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

    fn parse_rule(&self, p: &mut Parser) {
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
        m.complete(p, ValuePath);
    }
}
