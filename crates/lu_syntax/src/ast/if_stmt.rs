use log::debug;
use rowan::TextRange;

use crate::{AstElement, AstElementChildren, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ConditionElement, ElifKeywordToken, ElifOptKeywordToken,
    ElseKeywordToken, ElseStmtNode, IfElifStmtNode, IfKeywordToken, IfOptElifOptStmtNode,
    IfOptKeywordToken, NewlineToken, ValueExprElement, VarDeclNameToken,
};
use super::{IfElifElseStmtNode, IfElifElseStmtPartElement};

impl IfElifElseStmtNode {
    pub fn parts(&self) -> AstElementChildren<IfElifElseStmtPartElement> {
        support::element_children(self.syntax())
    }
}

impl IfElifElseStmtPartElement {
    pub fn fmt_for_debug(&self) -> String {
        match self {
            IfElifElseStmtPartElement::IfOptElifOptStmt(n) => n.fmt_for_debug(),
            IfElifElseStmtPartElement::IfElifStmt(n) => n.fmt_for_debug(),
            IfElifElseStmtPartElement::ElseStmt(n) => n.fmt_for_debug(),
        }
    }
}

impl IfElifStmtNode {
    pub fn fmt_for_debug(&self) -> String {
        let start = support::token_child::<IfKeywordToken>(self.syntax())
            .map(|n| n.syntax().text_range())
            .or(support::token_child::<ElifKeywordToken>(self.syntax())
                .map(|n| n.syntax().text_range()))
            .unwrap()
            .start();
        let end = self.condition().unwrap().syntax().text_range().end();
        self.text_at(&TextRange::new(start, end)).into()
    }
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
    pub fn fmt_cond_for_debug(&self) -> String {
        let start = self.var_name().unwrap().syntax().text_range().start();
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
    pub fn fmt_for_debug(&self) -> String {
        let start = support::token_child::<ElseKeywordToken>(self.syntax())
            .unwrap()
            .syntax()
            .text_range()
            .start();
        let end = support::token_child::<NewlineToken>(self.syntax())
            .unwrap()
            .syntax()
            .text_range()
            .end();
        self.text_at(&TextRange::new(start, end)).into()
    }
}
