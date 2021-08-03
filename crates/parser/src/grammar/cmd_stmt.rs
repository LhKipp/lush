use super::expr::ExpressionsRule;
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

pub(crate) struct CmdStmtRule;
impl Rule for CmdStmtRule {
    fn name(&self) -> String {
        "command".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == BareWord
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        //consume all ws delimited cmd's and arguments
        while p.eat(&[BareWord, Whitespace]) || { ExpressionsRule {}.opt(p).is_some() } {}
        Some(m.complete(p, CmdStmt))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_from_tokens, Event};

    use super::CmdStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/cmd_stmts.yaml")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_from_tokens(s, &CmdStmtRule {})
    }
}
