use super::*;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

pub struct ClsExprRule {}

impl Rule for ClsExprRule {
    fn matches(&self, p: &mut Parser) -> bool {
        let next = p.next_non(CMT_NL_WS);
        next == ClsKeyword || next == ImpureKeyword
    }

    fn name(&self) -> String {
        "ClsExpr".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_after(ImpureKeyword, CMT_NL_WS);
        p.expect_after(ClsKeyword, CMT_NL_WS);
        p.eat_while(CMT_NL_WS);

        SignatureRule {}.opt(p);
        BlockStmtRule::fn_for_block().parse(p);
        Some(m.complete(p, ClosureExpr))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::ClsExprRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/cls_expr/general.yaml_test")]
    fn parse_expr(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &ClsExprRule {})
    }
}
