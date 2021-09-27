use crate::{AstNode, AstNodeChildren, AstToken};

use super::{support, BareWordToken, LuTypeNode, StructFieldNode, StructNameToken, StructStmtNode};

impl StructStmtNode {
    pub fn name(&self) -> Option<String> {
        support::token_child::<StructNameToken>(self.syntax()).map(|n| n.text().to_string())
    }
    pub fn fields(&self) -> AstNodeChildren<StructFieldNode> {
        support::node_children(self.syntax())
    }
}

impl StructFieldNode {
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
