use crate::{AstNode, AstNodeChildren, AstToken};

use super::{support, BareWordToken, LuTypeNode, StrctFieldNode, StrctStmtNode, StructNameToken};

impl StrctStmtNode {
    pub fn name(&self) -> Option<String> {
        support::token_child::<StructNameToken>(self.syntax()).map(|n| n.text().to_string())
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
