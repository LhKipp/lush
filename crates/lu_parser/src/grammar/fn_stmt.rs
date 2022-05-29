use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS, CMT_WS},
    SyntaxKind::*,
};

pub struct FnStmtRule {}

impl Rule for FnStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        let next_token = p.next_non(CMT_NL_WS);
        next_token == FnKeyword || next_token == ImpureKeyword
    }

    fn name(&self) -> String {
        "FnStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_after(ImpureKeyword, CMT_NL_WS);
        p.expect_after(FnKeyword, CMT_NL_WS);
        p.eat_while(CMT_NL_WS);

        // eat the name
        p.eat_delimited_as(BareWord, FnDeclName, Whitespace, true);
        debug!("Testing for optional signature");
        SignatureRule {}.opt(p);
        p.expect_after(Newline, CMT_WS);
        debug!("Parsing fn_block");
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
