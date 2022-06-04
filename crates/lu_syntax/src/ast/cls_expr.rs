use lu_error::SourceCodeItem;
use rowan::TextRange;

use crate::{AstNode, AstToken};

use super::{
    addr_of_mod_node_contained_in, support, BlockStmtNode, ClosureExprNode, ClsKeywordToken,
    ImpureKeywordToken, SignatureNode,
};

impl ClosureExprNode {
    pub fn impure_attr(&self) -> Option<ImpureKeywordToken> {
        support::token_child(self.syntax())
    }

    pub fn fallback_in_ret_item(&self) -> SourceCodeItem {
        self.decl_item()
        // TODO evaluate whether below wouldnt be better
        // from first_name till end of last_name
        // let name_parts = support::token_children::<FnDeclNameToken>(self.syntax());
        // if let (Some(begin), Some(end)) = (name_parts.first(), name_parts.last()) {
        //     let range = TextRange::new(
        //         begin.syntax().text_range().start(),
        //         end.syntax().text_range().end(),
        //     );
        //     let text = self.text_at(&range);
        //     SourceCodeItem::new(range.into(), text)
        // } else {
        //     // Thats odd ... func without name
        //     support::token_child::<FnKeywordToken>(self.syntax())
        //         .unwrap()
        //         .into_item()
        // }
    }

    pub fn decl_item(&self) -> SourceCodeItem {
        // from fn till end of signature (or first item before)
        let cls_kw_range = support::token_child::<ClsKeywordToken>(self.syntax())
            .unwrap()
            .syntax()
            .text_range();
        let end = if let Some(sign) = self.signature() {
            sign.syntax().text_range().end()
        } else {
            cls_kw_range.end()
        };

        let text_range = TextRange::new(cls_kw_range.start(), end);
        let text = self.text_at(&text_range);

        SourceCodeItem::new(
            text_range.into(),
            text.to_string(),
            addr_of_mod_node_contained_in(self.syntax().clone()),
        )
    }

    pub fn signature(&self) -> Option<SignatureNode> {
        support::node_child(self.syntax())
    }

    pub fn block_stmt(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
