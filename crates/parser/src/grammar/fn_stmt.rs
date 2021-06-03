use super::*;

#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) fn expect_fn_stmt(p: &mut Parser) {
    p.eat_while(CMT_NL_WS);
    assert!(p.at(Fn));
    let m = p.start();
    p.eat(Fn);
    //consume all ws delimited bare words
    p.eat_while(&[BareWord, Whitespace]);
    p.eat_while(CMT_NL_WS);
    signature(p);
    newline(p);
    while p.eat_empty_or_cmt_line() {}
    block(p);

    m.complete(p, FnStmt);
}

fn signature(_p: &mut Parser) {
    todo!();
}
