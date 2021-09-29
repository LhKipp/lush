use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};
use vec_box::vec_box;

use super::{OrRule, Rule, SignatureRule};

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

pub struct FnTypeRule;
impl Rule for FnTypeRule {
    fn name(&self) -> String {
        "FnTypeRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == FnKeyword
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after(FnKeyword, CMT_NL_WS);

        SignatureRule {}.opt(p);
        Some(m.complete(p, FnType))
    }
}

fn lu_type_specifier() -> OrRule {
    OrRule {
        kind: Some("LuTypeSpecifier".to_string()),
        rules: vec_box![
            GenericType,
            NumberKeyword,
            AnyKeyword,
            NilKeyword,
            BoolKeyword,
            StringKeyword,
            FnTypeRule {},
            ArrayTypeRule {},
            StrctName,
            BareWord,
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
        // p.expect_after_as(BareWord, LuTypeName, CMT_NL_WS);
        lu_type_specifier().parse(p);
        // TODO make option type special in the language?
        // p.eat_after_as(T![?], OptModifier, CMT_NL_WS);
        Some(m.complete(p, LuType))
    }
}
