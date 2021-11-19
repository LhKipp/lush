#![allow(unused_imports)]
use crate::{AstNode, AstToken};

use super::{
    support, ArrayTypeNode, FnTypeNode, LuTypeNode, LuTypeSpecifierElement, OptModifierToken,
    SignatureNode,
};

impl LuTypeNode {
    pub fn type_specifier(&self) -> LuTypeSpecifierElement {
        support::element_child(self.syntax()).unwrap()
    }
    pub fn is_opt_type(&self) -> bool {
        support::token_child::<OptModifierToken>(self.syntax()).is_some()
    }
}

impl ArrayTypeNode {
    pub fn inner_type(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }
}

impl FnTypeNode {
    pub fn signature(&self) -> Option<SignatureNode> {
        support::node_child(self.syntax())
    }
}
