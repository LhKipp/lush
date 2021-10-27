use rowan::TextRange;

use crate::{AstElement, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ForKeywordToken, ForStmtNode, ValueExprElement, VarDeclNameToken,
};

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

    pub fn text_till_block(&self) -> String {
        let start = support::token_child::<ForKeywordToken>(self.syntax())
            .unwrap()
            .syntax()
            .text_range();
        let end = self
            .iterated_value()
            .expect("TODO take until end if no iterated_val")
            .syntax()
            .text_range();
        let range = TextRange::new(start.start(), end.end());
        self.text_at(&range).to_string()
    }
}
