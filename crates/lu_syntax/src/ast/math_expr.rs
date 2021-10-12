#![allow(unused_imports)]
use crate::{AstElement, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ConditionElement, ElifKeywordToken, IfStmtNode, MathExprNode,
    OperatorExprElement, ValueExprElement,
};

impl MathExprNode {
    pub fn lhs(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }

    pub fn rhs(&self) -> Option<ValueExprElement> {
        // TODO CHECK Is this always working?
        support::element_children(self.syntax()).skip(1).next()
    }

    pub fn operator(&self) -> OperatorExprElement {
        // Always some by parsing
        support::element_child(self.syntax()).unwrap()
    }
}
