use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct BlockStmtRule {
    pub parse_begin: bool,
    pub eat_end: bool,
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
            eat_end: true,
            end_kinds: [EndKeyword].into(),
            statement_rule: Box::new(second_level_stmt()),
        }
    }

    // New but with second_level_stmt rule as lazy
    pub fn new_lazy_rule() -> BlockStmtRule {
        BlockStmtRule {
            parse_begin: true,
            eat_end: true,
            end_kinds: [EndKeyword].into(),
            statement_rule: Box::new(Lazy::<OrRule>::new(|| second_level_stmt())),
        }
    }

    /// Source file block
    pub fn source_file_block() -> Self {
        Self {
            parse_begin: false,
            eat_end: false,
            end_kinds: [Eof].into(),
            statement_rule: Box::new(top_level_stmt()),
        }
    }

    /// BlockRule for if or elif blocks
    pub fn if_elif_block() -> Self {
        BlockStmtRule {
            parse_begin: false,
            eat_end: false,
            end_kinds: [ElseKeyword, ElifKeyword, EndKeyword].into(),
            statement_rule: Box::new(second_level_stmt()),
        }
    }

    pub fn else_block() -> Self {
        Self {
            parse_begin: false,
            eat_end: true,
            end_kinds: [EndKeyword].into(),
            statement_rule: Box::new(second_level_stmt()),
        }
    }

    pub fn fn_for_block() -> Self {
        Self {
            parse_begin: false,
            end_kinds: EndKeyword.into(),
            eat_end: true,
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

        while {
            // While not eof and not end keyword reached
            let next_non = p.next_non(CMT_NL_WS);
            next_non != Eof && !self.end_kinds.contains(next_non)
        } {
            self.statement_rule.parse(p);
        }

        p.eat_while(CMT_NL_WS);
        if self.eat_end {
            p.expect(self.end_kinds);
        }

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
        parse_as(s, &BlockStmtRule::new())
    }
}
