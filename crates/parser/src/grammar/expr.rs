#[allow(unused_imports)]
use super::*;

use crate::T;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) fn opt_value_expr(p: &mut Parser) -> bool {
    match p.current() {
        Number => expect_math_expr(p),
        Dollar => expect_value_path(p),
        SingleQuote | DoubleQuote => expect_string_expr(p),
        BareWord => expect_cmd_stmt(p),
        T!["["] => expect_table_or_array(p),
        _ => return false,
    }
    true
}

fn expect_table_or_array(p: &mut Parser) -> () {
    assert!(p.at(T!["["]));
    let m = p.start();
    let next = p.next_non(CMT_NL_WS);
    if next == T!["("] {
        expect_signature(p);
        // check whether table or array comes
    }
}

pub(crate) fn _value_expr(_p: &mut Parser) {}

pub(crate) fn expect_math_expr(p: &mut Parser) {
    // TODO for now just number
    assert!(p.at(Number));
    let m = p.start();
    p.eat(Number);
    m.complete(p, MathExpr);
}

pub(crate) fn expect_string_expr(p: &mut Parser) {
    assert!(p.at(&[DoubleQuote, SingleQuote]));
    let m = p.start();
    let quote_type = p.current();
    p.eat(p.current());
    p.eat_until(&[quote_type, Newline]);

    if p.current() == Newline {
        p.error("Unterminated string literal");
    }

    match quote_type {
        DoubleQuote => m.complete(p, DoubleQuotedString),
        SingleQuote => m.complete(p, SingleQuotedString),
        _ => unreachable!("quote type either double or signle"),
    };
}

pub(crate) fn expect_value_path(p: &mut Parser) {
    p.eat_while(CMT_NL_WS);
    assert!(p.at(Dollar));
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
