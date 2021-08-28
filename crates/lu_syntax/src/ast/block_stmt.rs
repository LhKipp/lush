use crate::{AstElementChildren, AstNode, AstNodeChildren};

use super::{support, BlockStmtNode, FnStmtNode, StatementElement};

impl BlockStmtNode {
    pub fn statements(&self) -> AstElementChildren<StatementElement> {
        support::element_children(self.syntax())
    }

    pub fn is_empty(&self) -> bool {
        self.statements().next().is_none()
    }

    pub fn fn_stmts(&self) -> AstNodeChildren<FnStmtNode> {
        support::node_children(self.syntax())
    }
}
