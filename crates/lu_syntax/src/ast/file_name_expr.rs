use std::path::PathBuf;

use crate::{AstElement, AstNode};

use super::{support, AbsFileNameNode, FileNameElement, FileNamePartElement, RelFileNameNode};

impl FileNameElement {
    pub fn path(&self) -> PathBuf {
        match self {
            FileNameElement::AbsFileName(abs) => abs.path(),
            FileNameElement::RelFileName(rel) => rel.path(),
        }
    }
}

impl RelFileNameNode {
    pub fn path(&self) -> PathBuf {
        support::element_children::<FileNamePartElement>(self.syntax())
            .map(|t| t.text())
            .collect()
    }
}
impl AbsFileNameNode {
    pub fn path(&self) -> PathBuf {
        support::element_children::<FileNamePartElement>(self.syntax())
            .map(|t| t.text())
            .collect()
    }
}
