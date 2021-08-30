use crate::grammar::LuTypeRule;
use crate::parser::CMT_WS;
use crate::token_set::TokenSet;
use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::*,
};

use super::Rule;

struct FlagSignatureRule;
impl Rule for FlagSignatureRule {
    fn name(&self) -> String {
        "FlagSignatureRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        let flag_kinds: TokenSet = [LongFlag, ShortFlag].into();
        flag_kinds.contains(p.next_non(CMT_NL_WS))
    }

    ///    --long_flag (-short_flag)? (<:> type)?
    ///    or
    ///    -short_flag (<:> type)?
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if p.eat_after(LongFlag, CMT_NL_WS) {
            p.eat_after(ShortFlag, CMT_WS); // shortflag belongs to longflag, must be on same line
        } else {
            // no long_flag, expect shortflag then (otherwise FlagSignatureRule wouldn't match)
            if !p.expect_after(ShortFlag, CMT_NL_WS) {
                m.abandon(p);
                return None;
            }
        }

        if p.eat_after(T![:], CMT_NL_WS) {
            LuTypeRule {}.parse(p);
        }

        Some(m.complete(p, FlagSignature))
    }
}

struct ParamSignatureRule {}
impl Rule for ParamSignatureRule {
    fn name(&self) -> String {
        "ParameterSignatureRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == BareWord
    }

    ///    name (<:> type)? (<?>)?
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after_as(BareWord, ParamName, CMT_NL_WS);
        p.eat_after_as(T![?], OptModifier, CMT_NL_WS);
        if p.eat_after(T![:], CMT_NL_WS) {
            LuTypeRule {}.parse(p);
        }
        Some(m.complete(p, ParamSignature))
    }
}

struct VarArgParamSignatureRule {}
impl Rule for VarArgParamSignatureRule {
    fn name(&self) -> String {
        "VarArgParamSignatureRule".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == VarArgName
    }

    ///    ...rest (<:> type)?
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.expect_after(CMT_NL_WS, VarArgName);
        if p.eat_after(T![:], CMT_NL_WS) {
            LuTypeRule {}.parse(p);
        }
        Some(m.complete(p, ParamSignature))
    }
}

pub struct SignatureRule;
impl Rule for SignatureRule {
    fn name(&self) -> String {
        "Signature".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["("]
    }

    /// [ parameter* rest_param? flag* ]
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();

        p.expect_after(CMT_NL_WS, T!["("]);

        let param_rule = ParamSignatureRule {};
        while param_rule.matches(p) {
            param_rule.parse(p);
        }

        VarArgParamSignatureRule {}.opt(p);

        let flag_rule = FlagSignatureRule {};
        while flag_rule.matches(p) {
            flag_rule.parse(p);
        }

        p.expect_after(CMT_NL_WS, T![")"]);
        Some(m.complete(p, Signature))
    }
}

#[cfg(test)]
mod tests {
    use pretty_env_logger::env_logger;

    use crate::{parse_as, Event};

    use super::SignatureRule;

    use {conformance, serde_yaml};

    #[conformance::tests(exact, serde=serde_yaml, file="test_data/grammar/signature/signature_simple.yaml_test")]
    fn parse_cmds(s: &str) -> Vec<Event> {
        let _ = env_logger::builder().is_test(true).try_init();
        parse_as(s, &SignatureRule {})
    }
}
