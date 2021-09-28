use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
    T,
};

struct StrctFieldRule;
impl Rule for StrctFieldRule {
    fn name(&self) -> String {
        "StructFieldRule".to_string()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == BareWord
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after(BareWord, CMT_NL_WS) {
            m.abandon(p);
            return None;
        }

        if p.eat_after(T![:], CMT_NL_WS) {
            LuTypeRule {}.parse(p);
        }

        Some(m.complete(p, StrctField))
    }
}

pub struct StrctStmtRule;
impl Rule for StrctStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == StrctKeyword
    }

    fn name(&self) -> String {
        "StructStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after(StrctKeyword, CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        p.expect_after(StructName, CMT_NL_WS);
        p.expect_after(T!["{"], CMT_NL_WS);

        //consume all <name: <Type>? args
        let field_rule = StrctFieldRule {};
        while field_rule.matches(p) {
            StrctFieldRule.parse(p);
        }

        p.expect_after(T!["}"], CMT_NL_WS);
        Some(m.complete(p, StrctStmt))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_as, Event};

    use super::StrctStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/struct_stmt/general.yaml_test")]
    fn parse(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &StrctStmtRule {})
    }
}
