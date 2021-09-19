use crate::{
    grammar::{LuTypeRule, OrRule, PipedCmdsStmtRule, ValueExprRule},
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
    T,
};
use vec_box::vec_box;

use super::Rule;

pub struct LetStmtRule;
impl Rule for LetStmtRule {
    fn name(&self) -> String {
        "let statement".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == LetKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_while(CMT_NL_WS);
        if !p.expect(LetKeyword) {
            // We better stop here, before doing more
            return None;
        }
        p.eat_while(CMT_NL_WS);
        p.expect_as(BareWord, VarDeclName);
        if p.eat_after(T![:], CMT_NL_WS) {
            LuTypeRule {}.parse(p);
        }
        p.expect_after(T![=], CMT_NL_WS);
        let let_rhs_rule = OrRule {
            kind: Some("LetRhsRule".into()),
            rules: vec_box![PipedCmdsStmtRule {}, ValueExprRule {}],
        };
        let_rhs_rule.parse(p);
        Some(m.complete(p, LetStmt))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_as, Event};

    use super::LetStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/let_stmt/let_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &LetStmtRule {})
    }
}
