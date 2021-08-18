use crate::T;
use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    SyntaxKind::Signature,
};

use super::Rule;

///This function is used to parse the parameter and flag list (signature)
///Such a signature can be of the following format:
/// [ (parameter | flag | rest_param | <eol>)* ]
///Where
///parameter is:
///    name (<:> type)? (<?>)? item_end
///flag is:
///    --name (-shortform)? (<:> type)? item_end
///rest is:
///    ...rest (<:> type)? item_end
///item_end:
///    (<,>)? (#Comment)? (<eol>)?
///
pub struct SignatureRule;
impl Rule for SignatureRule {
    fn name(&self) -> String {
        "Signature".into()
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.next_non(CMT_NL_WS) == T!["("]
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        p.eat_while(CMT_NL_WS);
        p.expect(T!["("]);
        p.expect(T![")"]);
        Some(m.complete(p, Signature))
    }
}
