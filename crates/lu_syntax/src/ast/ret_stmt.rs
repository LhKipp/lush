use crate::{ast::FnDeclNameToken, AstNode, AstToken};

use super::{support, BlockStmtNode, RetStmtNode, SignatureNode};

impl RetStmtNode {
    pub fn returned_val(&self) -> Option<ValueExprElement> {
        support::node_child(self.syntax())
    }
}
