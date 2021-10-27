#[rustfmt::skip]
pub(crate) mod nodes;
pub use nodes::*;

use crate::AstNode;

use super::support;

impl SourceFileNode {
    pub fn block(&self) -> Option<BlockStmtNode> {
        // TODO unwrap this. Always works
        support::node_child(self.syntax())
    }
}


