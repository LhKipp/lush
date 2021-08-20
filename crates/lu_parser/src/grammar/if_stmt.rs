use super::{condition::ConditionRule, statements_until, Rule};

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS, CMT_WS},
    SyntaxKind::*,
};

pub struct IfStmtRule;
impl Rule for IfStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == IfKeyword
    }

    fn name(&self) -> String {
        "IfStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let if_stmt_marker = p.start();
        p.eat_while(CMT_NL_WS);
        p.expect(IfKeyword);
        p.eat_while(CMT_NL_WS);
        ConditionRule::new().parse(p);
        p.expect_after(CMT_WS, Newline);

        let m = p.start();
        statements_until(p, [ElseKeyword, ElifKeyword, EndKeyword]);
        m.complete(p, IfBlock);

        while p.eat(ElifKeyword) {
            p.eat_while(CMT_NL_WS);
            ConditionRule::new().parse(p);
            p.expect_after(CMT_WS, Newline);

            let m = p.start();
            statements_until(p, [ElseKeyword, ElifKeyword, EndKeyword]);
            m.complete(p, ElifBlock);
        }

        p.eat_while(CMT_NL_WS);

        if p.eat(ElseKeyword) {
            p.eat_while(CMT_WS);
            p.expect_after(CMT_WS, Newline);

            let m = p.start();
            statements_until(p, EndKeyword);
            m.complete(p, ElseBlock);
        }

        p.expect(EndKeyword);

        Some(if_stmt_marker.complete(p, IfStmt))
    }
}

#[cfg(test)]
mod tests {
    // use lu_test_support::init_logger;
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::IfStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/if_stmt/single_if.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &IfStmtRule {})
    }
}
