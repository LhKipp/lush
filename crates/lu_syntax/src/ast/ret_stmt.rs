use crate::AstNode;

use super::{support, RetKeywordToken, RetStmtNode, ValueExprElement};

impl RetStmtNode {
    pub fn returned_val(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }
    pub fn ret_kw(&self) -> RetKeywordToken {
        support::token_child(self.syntax()).unwrap()
    }
}
