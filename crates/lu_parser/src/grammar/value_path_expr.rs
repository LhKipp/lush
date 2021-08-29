use super::*;

use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct ValuePathExprRule;
impl Rule for ValuePathExprRule {
    fn name(&self) -> String {
        "ValuePath".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T![$]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.eat_while(CMT_NL_WS);
        let m = p.start();
        p.expect(Dollar);
        loop {
            if !p.expect_as([BareWord, InKeyword], BareWord) {
                // in`$in` in is represented as in keyword
                break;
            }
            if !p.at(Point) {
                break;
            }
        }
        Some(m.complete(p, ValuePathExpr))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::ValuePathExprRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/value_path_expr/value_path_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &ValuePathExprRule {})
    }
}
