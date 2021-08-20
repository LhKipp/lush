use super::*;
use vec_box::vec_box;

use crate::parser::Parser;

pub struct ConditionRule {
    inner: OrRule,
}

impl ConditionRule {
    pub fn new() -> Self {
        Self {
            inner: OrRule {
                kind: Some("Condition".into()),
                rules: vec_box![CmdStmtRule {}, ValueExprRule {}],
            },
        }
    }
}

impl Rule for ConditionRule {
    fn name(&self) -> String {
        self.inner.name()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        self.inner.matches(p)
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        self.inner.parse_rule(p)
    }
}
