#![allow(unused_imports)]
use crate::{AstNode, AstToken};

use super::{
    support, ArrayTypeNode, FnTypeNode, LuTypeNode, LuTypeSpecifierElement, SignatureNode,
};

impl LuTypeNode {
    pub fn into_type(&self) -> LuTypeSpecifierElement {
        support::element_child(self.syntax()).unwrap()
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
