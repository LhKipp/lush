use std::path::PathBuf;

use crate::{AstNode};

use super::{AbsFileNameNode, FileNameElement, RelFileNameNode};

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
        self.text_trimmed().into()
    }
}
impl AbsFileNameNode {
    pub fn path(&self) -> PathBuf {
        self.text_trimmed().into()
    }
}
