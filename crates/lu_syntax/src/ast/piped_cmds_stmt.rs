use crate::{AstNode, AstNodeChildren};

use super::{support, CmdStmtNode, PipedCmdsStmtNode};

impl PipedCmdsStmtNode {
    pub fn cmds(&self) -> AstNodeChildren<CmdStmtNode> {
        support::node_children(self.syntax())
    }
}
