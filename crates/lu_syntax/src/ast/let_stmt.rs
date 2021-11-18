use lu_error::SourceCodeItem;
use rowan::TextRange;

use crate::{AstNode, AstToken};

use super::{
    addr_of_mod_node_contained_in, support, LetKeywordToken, LetStmtNode, LuTypeNode,
    ValueExprElement, VarDeclNameToken,
};

impl LetStmtNode {
    pub fn var_name(&self) -> Option<String> {
        support::token_child::<VarDeclNameToken>(self.syntax()).map(|t| t.text().to_string())
    }

    pub fn var_token(&self) -> Option<VarDeclNameToken> {
        support::token_child(self.syntax())
    }

    /// Returns the rhs of the assignment
    pub fn value(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }

    /// Returns the type of the declared variable
    pub fn decl_ty(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }

    pub fn item_till_assign(&self) -> SourceCodeItem {
        // We want to capture "let var : type"
        let start = support::token_child::<LetKeywordToken>(self.syntax())
            .unwrap()
            .syntax()
            .text_range();
        // Decly ty or first value before
        let end = if let Some(ty) = self.decl_ty() {
            ty.syntax().text_range()
        } else if let Some(name) = self.var_token() {
            name.syntax().text_range()
        } else {
            start.clone()
        };
        let text_range = TextRange::new(start.start(), end.end());
        let text = self.text_at(&text_range);

        SourceCodeItem::new(
            text_range.into(),
            text.to_string(),
            addr_of_mod_node_contained_in(self.syntax().clone()),
        )
    }
}
