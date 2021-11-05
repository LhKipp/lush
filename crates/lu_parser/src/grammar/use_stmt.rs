use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

use super::file_name_expr::file_name_rule;
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

        let file_name_rule = file_name_rule();
        if file_name_rule.matches(p) {
            file_name_rule.parse(p)
        } else {
            PluginUseStmtRule {}.parse(p)
        };
        Some(m.complete(p, UseStmt))
    }
}

pub struct PluginUseStmtRule {}
impl Rule for PluginUseStmtRule {
    fn name(&self) -> String {
        "PluginUseStmtRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == BareWord
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect(BareWord) {
            m.abandon(p);
            return None;
        }
        loop {
            if !p.eat_after(T![:], CMT_NL_WS) {
                break;
            }
            if !p.expect_after(BareWord, CMT_NL_WS) {
                break;
            }
        }
        Some(m.complete(p, PluginUseStmt))
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
