#[rustfmt::skip]
pub(crate) mod nodes;
pub use nodes::*;

use crate::AstNode;

use super::support;

impl SourceFileNode {
    pub fn statements(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}

impl ForStmtNode {
    /// The variables being declared in the for loop
    /// Example:
    /// for x in [] ...
    /// returns [x]
    pub fn var_names(&self) -> Vec<VarDeclNameToken> {
        support::token_children(self.syntax())
    }
    /// The value over which is iterated
    pub fn iterated_value(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }
    pub fn block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
