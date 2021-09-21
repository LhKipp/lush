use lu_error::SourceCodeItem;
use rowan::TextRange;

use crate::{ast::FnDeclNameToken, AstNode, AstToken};

use super::{support, BlockStmtNode, FnKeywordToken, FnStmtNode, SignatureNode};

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

    pub fn fallback_in_ret_item(&self) -> SourceCodeItem {
        let name_parts = support::token_children::<FnDeclNameToken>(self.syntax());
        if let (Some(begin), Some(end)) = (name_parts.first(), name_parts.last()) {
            let range = TextRange::new(
                begin.syntax().text_range().start(),
                end.syntax().text_range().end(),
            );
            let text = self.text_at(&range);
            SourceCodeItem::new(range.into(), text)
        } else {
            // Thats odd ... func without name
            support::token_child::<FnKeywordToken>(self.syntax())
                .unwrap()
                .into_item()
        }
    }

    pub fn signature(&self) -> Option<SignatureNode> {
        support::node_child(self.syntax())
    }

    pub fn block_stmt(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
