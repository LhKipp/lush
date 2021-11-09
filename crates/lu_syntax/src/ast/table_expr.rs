use crate::{AstElementChildren, AstNode, AstToken};

use super::{support, ArrayExprNode};

impl TableExprNode {
    pub fn tbl_signature(&self) -> TableSignatureNode {
        support::node_child(self.syntax())
    }

    pub fn rows(&self) -> AstNodeChildren<ArrayExprNode> {
        support::node_children(self.syntax())
    }
}
