use crate::{AstNode, AstToken};

use super::{support, IfStmtNode, ValueExprNode, VarDeclNameToken};

impl IfStmtNode {
    pub fn if_condition(&self) -> Option<ConditionNode> {
        support::token_child::<VarDeclNameToken>(self.syntax()).map(|t| t.text().to_string())
    }

    pub fn value(&self) -> Option<ValueExprNode> {
        support::element_child::<ValueExprNode>(self.syntax())
    }
}
