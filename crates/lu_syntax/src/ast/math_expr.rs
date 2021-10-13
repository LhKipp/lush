#![allow(unused_imports)]
use crate::{AstElement, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ConditionElement, ElifKeywordToken, IfStmtNode, MathExprNode,
    OperatorExprElement, ValueExprElement,
};

impl MathExprNode {
    pub fn lhs(&self) -> ValueExprElement {
        support::element_child(self.syntax()).unwrap()
    }

    pub fn rhs(&self) -> ValueExprElement {
        // TODO CHECK Is this always working?
        support::element_children(self.syntax())
            .skip(1)
            .next()
            .unwrap()
    }

    pub fn operator(&self) -> OperatorExprElement {
        // Always some by parsing
        support::element_child(self.syntax()).unwrap()
    }
}
