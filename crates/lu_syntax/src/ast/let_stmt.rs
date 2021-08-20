use crate::{AstNode, AstToken};

use super::{support, LetStmtNode, ValueExprNode, VarDeclNameToken};

impl LetStmtNode {
    pub fn var_name(&self) -> Option<String> {
        support::token_child::<VarDeclNameToken>(self.syntax()).map(|t| t.text().to_string())
    }

    pub fn value(&self) -> Option<ValueExprNode> {
        support::element_child::<ValueExprNode>(self.syntax())
    }
}
