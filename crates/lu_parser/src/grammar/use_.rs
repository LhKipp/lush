use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

use super::Rule;

pub struct UseStmtRule;
impl Rule for UseStmtRule {
    fn name(&self) -> String {
        "UseStmt".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == UseKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after(UseKeyword, CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        p.eat_while(CMT_NL_WS);
        p.expect(BareWord);
        loop {
            if !p.eat(T![:]) {
                break;
            }
            if !p.expect([BareWord, T![*]]) {
                break;
            }
        }

        Some(m.complete(p, UseStmt))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_as, Event};

    use super::UseStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/use_stmt/general.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &UseStmtRule {})
    }
}
