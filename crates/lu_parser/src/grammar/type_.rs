use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};
use vec_box::vec_box;

use super::{OrRule, Rule};

pub struct ArrayTypeRule;
impl Rule for ArrayTypeRule {
    fn name(&self) -> String {
        "ArrayType".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["["]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_while(CMT_NL_WS);
        p.expect(T!["["]);
        p.eat_while(CMT_NL_WS);
        LuTypeRule {}.opt(p);
        p.eat_while(CMT_NL_WS);
        p.expect(T!["]"]);
        Some(m.complete(p, ArrayType))
    }
}

fn lu_type_specifier() -> OrRule {
    OrRule {
        kind: Some("LuTypeSpecifier".to_string()),
        rules: vec_box![
            NumberKeyword,
            AnyKeyword,
            NilKeyword,
            BoolKeyword,
            NumberKeyword,
            StringKeyword,
            FnKeyword,
            ArrayTypeRule {},
        ],
    }
}

pub struct LuTypeRule;
impl Rule for LuTypeRule {
    fn name(&self) -> String {
        "LuTypeRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        lu_type_specifier().matches(p)
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();

        p.eat_while(CMT_NL_WS);
        lu_type_specifier().parse(p);
        p.eat_after_as(T![?], OptModifier, CMT_NL_WS);
        Some(m.complete(p, LuType))
    }
}
