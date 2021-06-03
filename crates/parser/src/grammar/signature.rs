#[allow(unused_imports)]
use super::*;
use log::debug;

use crate::T;
#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

fn opt_signature(p: &mut Parser) -> bool {
    if p.at(T!["("]) {
        expect_signature(p);
        true
    } else {
        false
    }
}

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
fn expect_signature(p: &mut Parser) {
    p.eat_while(CMT_NL_WS);
    debug!("Parsing signature");
    assert!(p.at(T!["("]));
    p.eat(T!["("]);
    p.expect(T![")"]);
}
