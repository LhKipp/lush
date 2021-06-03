#[allow(unused_imports)]
use super::*;
use crate::grammar::expr::opt_value_expr;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) fn expect_cmd_stmt(p: &mut Parser) {
    p.eat_while(CMT_NL_WS);
    assert!(p.at(BareWord));
    let m = p.start();
    //consume all ws delimited cmd's and arguments
    while p.eat(&[BareWord, Whitespace]) || opt_value_expr(p) {}
    m.complete(p, CmdStmt);
}
