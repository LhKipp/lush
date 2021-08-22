use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS, CMT_WS},
    SyntaxKind::*,
};

pub struct FnStmtRule;
impl Rule for FnStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == FnKeyword
    }

    fn name(&self) -> String {
        "FnStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.eat(FnKeyword);
        //consume all ws delimited bare words
        p.eat_while(&[BareWord, Whitespace]);
        p.eat_while(CMT_NL_WS);
        SignatureRule {}.opt(p);
        p.expect_after(CMT_WS, Newline);
        BlockStmtRule {
            parse_begin: false,
            end_kinds: EndKeyword.into(),
            statement_rule: Box::new(second_level_stmt()),
        }
        .parse(p);
        Some(m.complete(p, FnStmt))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::FnStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/fn_stmt/fn_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &FnStmtRule {})
    }
}
