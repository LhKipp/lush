use super::{CmdStmtRule, Rule};
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
    T,
};

pub struct PipedCmdsStmtRule;
impl Rule for PipedCmdsStmtRule {
    fn name(&self) -> String {
        "piped commands".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        CmdStmtRule {}.matches(p)
    }

    /// If no | is being detected, this returns the CmdStmtRule marker
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        let mut pipe_detected = false;

        loop {
            let result = CmdStmtRule {}.parse(p);

            if p.next_non(CMT_NL_WS) == T![|] {
                pipe_detected = true;
                p.eat_while(CMT_NL_WS);
                p.expect(T![|]);
            } else {
                // No | --> end detected
                if pipe_detected {
                    // at least one |, return proper pipe-rule
                    return Some(m.complete(p, PipedCmdsStmt));
                } else {
                    // no |, abandon this rule and only return the parsed cmd stmt
                    m.abandon(p);
                    return result;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::PipedCmdsStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/piped_cmds_stmt/piped_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &PipedCmdsStmtRule {})
    }
}