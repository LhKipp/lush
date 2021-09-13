#![allow(unused_imports)]
use crate::{AstNode, AstToken};

use super::{support, ArrayTypeNode, LuTypeNode, LuTypeSpecifierElement};

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
