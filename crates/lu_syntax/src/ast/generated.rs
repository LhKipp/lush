#[rustfmt::skip]
pub(crate) mod nodes;
pub use nodes::*;

use crate::AstNode;

use super::support;

impl SourceFileNode {
    pub fn block(&self) -> BlockStmtNode {
        support::node_child(self.syntax()).unwrap()
    }
}
