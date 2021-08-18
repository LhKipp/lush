use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

use super::Rule;

pub(crate) struct LetStmtRule;
impl Rule for LetStmtRule {
    fn name(&self) -> String {
        "let statement".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == LetKeyword
    }

    fn parse_rule(&self, _p: &mut Parser) -> Option<CompletedMarker> {
        todo!();
    }
}
