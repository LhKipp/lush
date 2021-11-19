use log::debug;
use rowan::TextRange;

use crate::{AstElement, AstElementChildren, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ConditionElement, ElifOptKeywordToken, ElseStmtNode, IfElifStmtNode,
    IfOptElifOptStmtNode, IfOptKeywordToken, ValueExprElement, VarDeclNameToken,
};
use super::{IfElifElseStmtNode, IfElifElseStmtPartElement};

impl IfElifElseStmtNode {
    pub fn parts(&self) -> AstElementChildren<IfElifElseStmtPartElement> {
        support::element_children(self.syntax())
    }
}

impl IfElifStmtNode {
    pub fn condition(&self) -> Option<ConditionElement> {
        debug!("{:#?}", self);
        support::element_child(self.syntax())
    }
    pub fn block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
impl IfOptElifOptStmtNode {
    pub fn fmt_for_debug(&self) -> String {
        let start = support::token_child::<IfOptKeywordToken>(self.syntax())
            .map(|n| n.syntax().text_range())
            .or(support::token_child::<ElifOptKeywordToken>(self.syntax())
                .map(|n| n.syntax().text_range()))
            .unwrap()
            .start();
        let end = self.rhs_opt().unwrap().syntax().text_range().end();
        self.text_at(&TextRange::new(start, end)).into()
    }

    pub fn block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
    pub fn var_name(&self) -> Option<VarDeclNameToken> {
        support::token_child(self.syntax())
    }
    pub fn rhs_opt(&self) -> Option<ValueExprElement> {
        support::element_child(self.syntax())
    }
}

impl ElseStmtNode {
    pub fn block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
