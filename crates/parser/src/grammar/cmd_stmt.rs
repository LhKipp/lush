#[allow(unused_imports)]
use super::*;
use crate::grammar::expr::value_expr_rule;
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

    fn parse_rule(&self, p: &mut Parser) {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        //consume all ws delimited cmd's and arguments
        while p.eat(&[BareWord, Whitespace]) || { value_expr_rule().opt(p) } {}
        m.complete(p, CmdStmt);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_from_tokens, Event};

    use super::CmdStmtRule;

    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/cmd_stmts.json")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        parse_from_tokens(s, &CmdStmtRule {})
    }
}
