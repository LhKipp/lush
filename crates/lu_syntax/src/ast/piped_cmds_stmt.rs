use crate::{AstElementChildren, AstNode};

use super::{support, PipedCmdsStmtNode, ValueExprElement};

impl PipedCmdsStmtNode {
    pub fn piped_args(&self) -> AstElementChildren<ValueExprElement> {
        support::element_children(self.syntax())
    }
}
