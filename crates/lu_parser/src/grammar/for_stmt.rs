use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct ForStmtRule;
impl Rule for ForStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == ForKeyword
    }

    fn name(&self) -> String {
        "ForStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.expect(ForKeyword);
        p.eat_while(CMT_NL_WS);
        p.expect(BareWord); // There should be at least 1 var being declared

        //consume all ws delimited bare words
        p.eat_while(&[BareWord, Whitespace]);
        p.eat_while(CMT_NL_WS);
        p.expect(InKeyword);
        p.eat_while(CMT_NL_WS);
        ValueExprRule {}.parse(p);
        p.eat_until(Newline);
        p.expect(Newline);
        block(p);
        Some(m.complete(p, ForStmt))
    }
}

#[cfg(test)]
mod tests {
    // use lu_test_support::init_logger;
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::ForStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/for_stmt/for_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &ForStmtRule {})
    }
}
