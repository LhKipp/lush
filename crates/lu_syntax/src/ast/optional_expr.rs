use crate::AstNode;

use super::{support, OptionalExprNode, ValueExprElement};

impl OptionalExprNode {
    pub fn value(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }
}
