#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) struct LetStmtRule;
impl Rule for LetStmtRule {
    fn name(&self) -> String {
        "let statement".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == Let
    }

    fn parse_rule(&self, _p: &mut Parser) {
        todo!();
    }
}
