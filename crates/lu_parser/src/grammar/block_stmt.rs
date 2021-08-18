use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

fn block_stmt() -> OrRule {
    OrRule {
        kind: None,
        rules: vec![Box::new(LetStmtRule {}), Box::new(CmdStmtRule {})],
    }
}

pub struct BlockStmtRule;
impl Rule for BlockStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == BeginKeyword
    }

    fn name(&self) -> String {
        "BlockStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect(BeginKeyword);
        while p.next_non(CMT_NL_WS) != EndKeyword {
            block_stmt().parse(p);
        }
        p.eat_while(CMT_NL_WS);
        p.expect(EndKeyword);
        Some(m.complete(p, BlockStmt))
    }
}

#[cfg(test)]
mod tests {
    use lu_test_support::init_logger;

    use crate::{parse_as, Event};

    use super::BlockStmtRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/block_stmt/block_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        init_logger();
        parse_as(s, &BlockStmtRule {})
    }
}
