use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct BlockStmtRule {
    pub parse_begin: bool,
    pub end_kinds: TokenSet,
    pub statement_rule: Box<dyn Rule>,
}

impl BlockStmtRule {
    /// Returns default block rule
    /// begin
    ///     <stmts>
    /// end
    pub fn new() -> BlockStmtRule {
        BlockStmtRule {
            parse_begin: true,
            end_kinds: [EndKeyword].into(),
            statement_rule: Box::new(second_level_stmt()),
        }
    }
}

impl Rule for BlockStmtRule {
    fn matches(&self, p: &mut Parser) -> bool {
        assert!(self.parse_begin);
        p.next_non(CMT_NL_WS) == BeginKeyword
    }

    fn name(&self) -> String {
        "BlockStmt".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();

        if self.parse_begin {
            p.expect(BeginKeyword);
        }

        while !self.end_kinds.contains(p.next_non(CMT_NL_WS)) {
            self.statement_rule.parse(p);
        }

        p.eat_while(CMT_NL_WS);
        p.expect(self.end_kinds);

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
