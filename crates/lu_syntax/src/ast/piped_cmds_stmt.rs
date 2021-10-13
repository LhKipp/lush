use crate::{AstElementChildren, AstNode};

use super::{support, CmdOrValueExprElement, PipedCmdsStmtNode};

impl PipedCmdsStmtNode {
    pub fn piped_args(&self) -> AstElementChildren<CmdOrValueExprElement> {
        support::element_children(self.syntax())
    }
}
