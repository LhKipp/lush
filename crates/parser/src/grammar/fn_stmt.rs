use super::{signature::SignatureRule, Rule, *};

#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) struct FnStmtRule;
impl Rule for FnStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == Fn
    }

    fn name(&self) -> String {
        "FnStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.eat(Fn);
        //consume all ws delimited bare words
        p.eat_while(&[BareWord, Whitespace]);
        p.eat_while(CMT_NL_WS);
        SignatureRule {}.opt(p);
        p.eat_while(CMT_NL_WS);
        block(p);

        Some(m.complete(p, FnStmt))
    }
}
