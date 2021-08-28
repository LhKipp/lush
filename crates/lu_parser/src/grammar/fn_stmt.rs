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
        let m = p.start();
        p.eat_while(CMT_NL_WS);

        p.expect(FnKeyword);
        p.eat_while(CMT_NL_WS);

        // eat the name
        p.eat_delimited_as(BareWord, FnDeclName, Whitespace, true);
        p.eat_while(CMT_NL_WS);

        SignatureRule {}.opt(p);
        p.expect_after(CMT_WS, Newline);
        BlockStmtRule::fn_for_block().parse(p);
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
