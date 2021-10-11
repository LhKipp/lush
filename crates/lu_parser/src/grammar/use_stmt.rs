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

        let use_path_part_rule = FileNameRule {};
        use_path_part_rule.parse(p);
        loop {
            if !p.eat(T![:]) {
                break;
            }
            if use_path_part_rule.parse(p).is_none() {
                break;
            }
        }

        Some(m.complete(p, UseStmt))
    }
}

pub struct FileNameRule {}
impl Rule for FileNameRule {
    fn name(&self) -> String {
        "FileNameRule".to_string()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let next_token = p.next_non(CMT_NL_WS);
        next_token == BareWord || next_token == T![.]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        if !self.matches(p) {
            return None;
        }

        let m = p.start();
        loop {
            let eaten = if p.eat(T![.]) {
                p.eat(BareWord)
            } else {
                p.eat(BareWord)
            };
            if !eaten {
                break;
            }
        }
        Some(m.complete(p, FileName))
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
