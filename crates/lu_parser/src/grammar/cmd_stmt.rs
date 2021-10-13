use log::debug;

use super::{expr::ValueExprRule, Rule};
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct CmdStmtRule;
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
        let arg_rule = ValueExprRule {};
        loop {
            debug!("CmdStmtRule checking for arg or cmd_name");
            let next_token = p.next_non(Whitespace);
            if next_token == Eof || next_token == Newline {
                break;
            }
            if p.eat_after(BareWord, Whitespace) {
                continue;
            } else if arg_rule.opt(p).is_some() {
                continue;
            } else {
                debug!("Breaking cmd stmt");
                break;
            }
        }
        Some(m.complete(p, CmdStmt))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::CmdStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/cmd_stmt/cmd_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &CmdStmtRule {})
    }
}
