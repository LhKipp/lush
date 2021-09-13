use crate::{ast::FnDeclNameToken, AstNode, AstToken};

use super::{support, BlockStmtNode, FnStmtNode, SignatureNode};

impl FnStmtNode {
    pub fn name(&self) -> Option<String> {
        let name_parts = support::token_children::<FnDeclNameToken>(self.syntax());
        if name_parts.is_empty() {
            None
        } else {
            Some(
                name_parts
                    .iter()
                    .map(|n| n.text())
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
    }

    pub fn signature(&self) -> Option<SignatureNode> {
        support::node_child(self.syntax())
    }

    pub fn block_stmt(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
