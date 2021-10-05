use std::path::PathBuf;

use crate::ast::UsePathElement;
use crate::{AstElement, AstElementChildren, AstNode};

use super::{support, UseStmtNode};

impl UseStmtNode {
    pub fn is_std_path(&self) -> bool {
        self.path()
            .next()
            .map(|part| part.text() == "std")
            .unwrap_or(false)
    }

    pub fn path(&self) -> AstElementChildren<UsePathElement> {
        support::element_children(self.syntax())
    }

    pub fn path_as_path_buf(&self) -> PathBuf {
        let path_as_str = self
            .path()
            .map(|path| path.text())
            .collect::<Vec<_>>()
            .join("/");
        PathBuf::from(path_as_str)
    }
    pub fn parts(&self) -> Vec<String> {
        self.path().map(|n|n.text()).collect()
    }
}
