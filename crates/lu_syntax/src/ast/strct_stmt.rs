use crate::{AstNode, AstNodeChildren, AstToken};

use super::{
    support, BareWordToken, CmdOrValueExprElement, LuTypeNode, StrctCtorExprNode,
    StrctFieldCtorStmtNode, StrctFieldNameToken, StrctFieldNode, StrctNameToken, StrctStmtNode,
};

impl StrctStmtNode {
    pub fn name(&self) -> Option<String> {
        support::token_child::<StrctNameToken>(self.syntax()).map(|n| n.text().to_string())
    }
    pub fn fields(&self) -> AstNodeChildren<StrctFieldNode> {
        support::node_children(self.syntax())
    }
}

impl StrctFieldNode {
    pub fn name(&self) -> String {
        support::token_child::<BareWordToken>(self.syntax())
            .expect("Always Some")
            .text()
            .to_string()
    }

    pub fn ty(&self) -> Option<LuTypeNode> {
        support::node_child::<LuTypeNode>(self.syntax())
    }
}

impl StrctCtorExprNode {
    pub fn name(&self) -> String {
        support::token_child::<StrctNameToken>(self.syntax())
            .expect("Always Some")
            .text()
            .to_string()
    }
    pub fn fields(&self) -> AstNodeChildren<StrctFieldCtorStmtNode> {
        support::node_children(self.syntax())
    }
}

impl StrctFieldCtorStmtNode {
    pub fn field_name(&self) -> String {
        support::token_child::<StrctFieldNameToken>(self.syntax())
            .expect("Always Some")
            .text()
            .to_string()
    }

    /// Returns the rhs of the assignment
    pub fn value(&self) -> Option<CmdOrValueExprElement> {
        support::element_child(self.syntax())
    }
}
