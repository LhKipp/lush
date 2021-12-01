use super::{BlockStmtRule, OrRule, Rule};
use vec_box::vec_box;

use crate::{
    grammar::{LuTypeRule, PipedCmdsStmtRule, ValueExprRule},
    parser::{CompletedMarker, Parser, CMT_NL_WS, CMT_WS},
    SyntaxKind::{self, *},
    T,
};

pub struct IfElifElseRule;
impl Rule for IfElifElseRule {
    fn name(&self) -> String {
        "IfStmt".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let next = p.next_non(CMT_NL_WS);
        next == IfKeyword || next == IfOptKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();

        let if_rules = OrRule {
            rules: vec_box![
                IfElifOptRulePart {
                    to_parse: IfKeyword
                },
                IfElifOptRulePart {
                    to_parse: IfOptKeyword
                }
            ],
            kind: None,
        };
        let elif_rules = OrRule {
            rules: vec_box![
                IfElifOptRulePart {
                    to_parse: ElifKeyword
                },
                IfElifOptRulePart {
                    to_parse: ElifOptKeyword
                }
            ],
            kind: None,
        };
        let else_rule = ElseRule {};

        if if_rules.parse(p).is_none() {
            m.abandon(p);
            return None;
        }

        while elif_rules.matches(p) {
            elif_rules.parse(p);
        }

        if else_rule.matches(p) {
            else_rule.parse(p);
        } else {
            p.expect_after(EndKeyword, CMT_NL_WS);
        }

        Some(m.complete(p, IfElifElseStmt))
    }
}

struct IfElifOptRulePart {
    to_parse: SyntaxKind,
}
impl Rule for IfElifOptRulePart {
    fn name(&self) -> String {
        self.to_parse.to_string()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == self.to_parse
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after(self.to_parse, CMT_NL_WS);

        match self.to_parse {
            IfKeyword | ElifKeyword => {
                // The condition
                PipedCmdsStmtRule {}.parse(p);
            }
            IfOptKeyword | ElifOptKeyword => {
                p.expect_after_as(BareWord, VarDeclName, CMT_NL_WS);
                if p.eat_after(T![:], CMT_NL_WS) {
                    LuTypeRule {}.parse(p);
                }
                if p.expect_after(T![=], CMT_NL_WS) {
                    ValueExprRule {}.parse(p); // TODO should be piped_cmds_stmt
                }
            }
            _ => unreachable!(),
        }
        p.expect_after(Newline, CMT_WS);
        BlockStmtRule::if_elif_elifopt_block().parse(p);

        match self.to_parse {
            IfKeyword | ElifKeyword => Some(m.complete(p, IfElifStmt)),
            IfOptKeyword | ElifOptKeyword => Some(m.complete(p, IfOptElifOptStmt)),
            _ => unreachable!(),
        }
    }
}

struct ElseRule;
impl Rule for ElseRule {
    fn name(&self) -> String {
        "else".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == ElseKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after(ElseKeyword, CMT_NL_WS);
        p.expect_after(Newline, CMT_WS);
        BlockStmtRule::else_block().parse(p);
        Some(m.complete(p, ElseStmt))
    }
}

#[cfg(test)]
mod tests {
    // use lu_test_support::init_logger;
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::IfElifElseRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/if_stmt/single_if.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &IfElifElseRule {})
    }
}
