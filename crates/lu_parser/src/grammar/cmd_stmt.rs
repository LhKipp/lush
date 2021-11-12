use log::debug;

use super::{expr::ValueExprRule, file_name_expr::file_name_rule, Rule};
use crate::{
    grammar::{OrRule, ValuePathExprRule},
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
    T,
};
use vec_box::vec_box;

pub struct RedirStmt {}
impl Rule for RedirStmt {
    fn name(&self) -> String {
        "RedirStmt".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T![>>]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if !p.expect_after(T![>>], CMT_NL_WS) {
            m.abandon(p);
            return None;
        }
        let redir_to = OrRule {
            kind: Some("RedirToValue".to_string()),
            rules: vec_box![ValuePathExprRule {}, BareWord],
        };
        redir_to.parse(p);
        Some(m.complete(p, RedirStmt))
    }
}

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
        // Eat commands name
        if !p.expect_after(BareWord, CMT_NL_WS) {
            m.abandon(p);
            return None;
        }

        //consume all ws delimited cmd's and arguments
        let arg_rule = ValueExprRule {};
        let file_name_rule = file_name_rule(true);
        loop {
            debug!("CmdStmtRule checking for arg or cmd_name");
            let next_token = p.next_non(Whitespace);
            if next_token == Eof || next_token == Newline {
                p.eat_until(&[Eof, Newline].into());
                break;
            }
            if
            // Give BareWord precedence over ValueExprRule CmdStmt
            // We allow simple barewords as cmdarg
            // But filenames still require precedence over barewords...
            // TODO integrate bareword optionally into ValueExpr
            file_name_rule.opt(p).is_some()
                || p.eat_after(BareWord, Whitespace)
                || arg_rule.opt(p).is_some()
                || p.eat_after(ShortFlag, Whitespace)
                || p.eat_after(LongFlag, Whitespace)
            {
                continue;
            } else {
                break;
            }
        }

        RedirStmt {}.opt(p);

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
