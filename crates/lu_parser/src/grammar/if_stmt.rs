use super::{condition::ConditionRule, BlockStmtRule, Rule};

use crate::{
    self,
    parser::{CompletedMarker, Parser, CMT_NL_WS, CMT_WS},
    *,
};

/// helper for parsing if / elif stmts
fn parse_with_cond(kind: SyntaxKind, p: &mut Parser) {
    assert!(kind == SyntaxKind::IfKeyword || kind == SyntaxKind::ElifKeyword);
    let if_elif_block_rule = BlockStmtRule::if_elif_block();

    p.expect(kind);
    p.eat_while(CMT_NL_WS);
    ConditionRule::new().parse(p);
    p.expect_after(CMT_WS, Newline);

    if_elif_block_rule.parse(p);
}

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

        parse_with_cond(IfKeyword, p);

        while p.at(ElifKeyword) {
            parse_with_cond(ElifKeyword, p);
        }

        if p.eat(ElseKeyword) {
            p.eat_while(CMT_WS);
            p.expect_after(CMT_WS, Newline);

            BlockStmtRule::else_block().parse(p);
        }

        // if / elif does not eat end keyword. Do so here
        if p.eat(EndKeyword) {
            Some(if_stmt_marker.complete(p, IfStmt))
        }

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
