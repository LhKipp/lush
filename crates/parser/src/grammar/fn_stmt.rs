use super::*;
use log::debug;

use crate::T;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) fn expect_fn_stmt(p: &mut Parser) {
    debug!("Parsing fn_stmt");
    p.eat_while(CMT_NL_WS);
    assert!(p.at(Fn));
    let m = p.start();
    p.eat(Fn);
    //consume all ws delimited bare words
    p.eat_while(&[BareWord, Whitespace]);
    p.eat_while(CMT_NL_WS);
    opt_signature(p);
    p.eat_while(CMT_NL_WS);
    block(p);

    m.complete(p, FnStmt);
}
