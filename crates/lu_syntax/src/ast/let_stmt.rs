use crate::{AstNode, AstToken};

use super::{support, LetStmtNode, ValueExprElement, VarDeclNameToken};

impl LetStmtNode {
    pub fn var_name(&self) -> Option<String> {
        support::token_child::<VarDeclNameToken>(self.syntax()).map(|t| t.text().to_string())
    }

    pub fn value(&self) -> Option<ValueExprElement> {
        support::element_child::<ValueExprElement>(self.syntax())
    }
}
