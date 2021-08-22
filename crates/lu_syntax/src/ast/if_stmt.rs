use crate::{AstElement, AstNode, AstToken};

use super::{support, BlockStmtNode, ConditionElement, ElifKeywordToken, IfStmtNode};

impl IfStmtNode {
    pub fn if_condition(&self) -> Option<ConditionElement> {
        support::element_child::<ConditionElement>(self.syntax())
    }

    pub fn if_block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }

    pub fn elif_blocks(&self) -> Vec<(Option<ConditionElement>, Option<BlockStmtNode>)> {
        let elifs = support::token_children::<ElifKeywordToken>(self.syntax());
        elifs
            .iter()
            .map(|elif| {
                // TODO the next sipling does not necessarily have to be the cond / statements
                // TODO impl find_until([ElifKeyword, ElseKeyword, End])
                let cond = elif.syntax().next_sibling_or_token();
                let statements = cond
                    .as_ref()
                    .map(|cond| cond.next_sibling_or_token())
                    .flatten();
                (
                    cond.map(ConditionElement::cast).flatten(),
                    statements
                        .map(|stmts| BlockStmtNode::cast_element(stmts))
                        .flatten(),
                )
            })
            .collect()
    }

    pub fn else_block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }
}
