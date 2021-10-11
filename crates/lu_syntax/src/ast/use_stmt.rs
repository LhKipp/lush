use crate::ast::UsePathElement;
use crate::{AstElement, AstElementChildren, AstNode};

use super::{support, FileNameNode, UseStmtNode};

impl UseStmtNode {
    pub fn is_std_path(&self) -> bool {
        self.path()
            .next()
            .map(|part| part.text() == "std")
            .unwrap_or(false)
    }
    pub fn is_file_path(&self) -> bool {
        true // TODO
    }
    pub fn is_plugin_path(&self) -> bool {
        let path: Vec<_> = self.path().collect();
        path.len() == 1
            || path
                .get(1)
                .map(|n| matches!(n, UsePathElement::DoublePoint(_)))
                .unwrap_or(false)
    }
    pub fn parts(&self) -> Vec<String> {
        support::node_children::<FileNameNode>(self.syntax())
            .map(|n| n.text())
            .collect()
    }

    pub fn path(&self) -> AstElementChildren<UsePathElement> {
        support::element_children(self.syntax())
    }
}
