use crate::ast::UsePathElement;
use crate::{AstElementChildren, AstNode};

use super::{support, UseStmtNode};

impl UseStmtNode {
    pub fn path(&self) -> AstElementChildren<UsePathElement> {
        support::element_children(self.syntax())
    }
}
