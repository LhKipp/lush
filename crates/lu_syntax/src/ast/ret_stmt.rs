use crate::AstNode;

use super::{support, PipeOrValueExprElement, RetKeywordToken, RetStmtNode};

impl RetStmtNode {
    pub fn returned_val(&self) -> Option<PipeOrValueExprElement> {
        support::element_child(self.syntax())
    }
    pub fn ret_kw(&self) -> RetKeywordToken {
        support::token_child(self.syntax()).unwrap()
    }
}
