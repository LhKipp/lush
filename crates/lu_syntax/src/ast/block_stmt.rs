use crate::{AstElementChildren, AstNode};

use super::{support, BlockStmtNode, StatementElement};

impl BlockStmtNode {
    pub fn statements(&self) -> AstElementChildren<StatementElement> {
        support::element_children(self.syntax())
    }

    pub fn is_empty(&self) -> bool {
        self.statements().next().is_none()
    }
}
