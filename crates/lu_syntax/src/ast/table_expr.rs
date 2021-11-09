use crate::{AstNode, AstNodeChildren};

use super::{support, ArrayExprNode, StrctNameToken, TableExprNode};

impl TableExprNode {
    pub fn strct_name(&self) -> Option<StrctNameToken> {
        support::token_child(self.syntax())
    }

    pub fn rows(&self) -> AstNodeChildren<ArrayExprNode> {
        support::node_children(self.syntax())
    }
}
