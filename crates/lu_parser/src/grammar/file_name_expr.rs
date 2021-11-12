use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};
use vec_box::vec_box;

use super::{OrRule, Rule};

pub(crate) fn file_name_rule(allow_wildcards: bool) -> OrRule {
    OrRule {
        kind: Some("FileName".into()),
        rules: vec_box![
            AbsFileNameRule { allow_wildcards },
            RelFileNameRule { allow_wildcards }
        ],
    }
}

pub struct RelFileNameRule {
    allow_wildcards: bool,
}

impl Rule for RelFileNameRule {
    fn name(&self) -> String {
        "RelFileNameRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let next_token = p.next_non(CMT_NL_WS);
        if next_token == BareWord {
            // FileSep has to come right after
            let token_after_bw = p.next_token_after(BareWord);
            token_after_bw == T![/]
        } else if next_token == T![*] && self.allow_wildcards {
            true
        } else {
            // ./<bw> rule
            next_token == T![.]
        }
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if p.eat_after(T![.], CMT_NL_WS) {
            if !p.expect(T![/]) {
                m.abandon(p);
                return None;
            }
        } else {
            p.eat_while(CMT_NL_WS);
        }
        loop {
            if !p.eat_while_file_name_elem(self.allow_wildcards) {
                break;
            }
            if !p.eat(T![/]) {
                break;
            }
        }
        Some(m.complete(p, RelFileName))
    }
}
pub struct AbsFileNameRule {
    allow_wildcards: bool,
}
impl Rule for AbsFileNameRule {
    fn name(&self) -> String {
        "AbsFileNameRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T![/]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after(T![/], CMT_NL_WS);
        loop {
            if !p.eat_while_file_name_elem(self.allow_wildcards) {
                break;
            }
            if !p.eat(T![/]) {
                break;
            }
        }
        Some(m.complete(p, AbsFileName))
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_as, Event, SourceFileRule};

    use super::file_name_rule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/file_name_expr/general.yaml_test")]
    fn parse_without_wildcard(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &file_name_rule(false))
    }
    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/file_name_expr/with_wildcards.yaml_test")]
    fn parse_with_wildcards(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &file_name_rule(true))
    }
    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/file_name_expr/as_cmd_arg.yaml_test")]
    fn parse_as_cmd_arg(s: &str) -> Vec<Event> {
        lu_test_support::init_logger();
        parse_as(s, &SourceFileRule::default())
    }
}
