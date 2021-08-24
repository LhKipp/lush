use crate::{AstElement, AstNode, AstToken};

use super::{
    support, BlockStmtNode, ConditionElement, ElifKeywordToken, ElseKeywordToken, IfStmtNode,
};

impl IfStmtNode {
    pub fn if_condition(&self) -> Option<ConditionElement> {
        support::element_child::<ConditionElement>(self.syntax())
    }

    pub fn if_block(&self) -> Option<BlockStmtNode> {
        support::node_child(self.syntax())
    }

    pub fn elif_blocks(&self) -> Vec<(Option<ConditionElement>, Option<BlockStmtNode>)> {
        let mut result = Vec::new();
        let elems: Vec<_> = self.syntax().children_with_tokens().collect();
        for i in 0..elems.len() {
            let elem = &elems[i];
            if ElifKeywordToken::can_cast(elem.kind()) {
                let cond = elems[(i + 1)..elems.len()]
                    .iter()
                    // TODO performance filter based on kind first
                    .find_map(|n| ConditionElement::cast(n.clone()));
                let block = elems[(i + 1)..elems.len()]
                    .iter()
                    .find_map(|n| BlockStmtNode::cast_element(n.clone()));
                result.push((cond, block))
            }
        }
        result
    }

    pub fn else_block(&self) -> Option<BlockStmtNode> {
        let mut after_else = self
            .syntax()
            .children_with_tokens()
            .skip_while(|n| !ElseKeywordToken::can_cast(n.kind()))
            .skip(1);
        after_else.find_map(BlockStmtNode::cast_element)
    }
}
