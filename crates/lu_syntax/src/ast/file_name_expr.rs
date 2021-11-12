use glob::Pattern;
use std::path::PathBuf;

use lu_error::{AstErr, LuErr};

use crate::{AstElement, AstNode};

use super::{AbsFileNameNode, FileNameElement, RelFileNameNode};

impl FileNameElement {
    pub fn path(&self) -> PathBuf {
        match self {
            FileNameElement::AbsFileName(abs) => abs.path(),
            FileNameElement::RelFileName(rel) => rel.path(),
        }
    }
    pub fn value(&self) -> String {
        self.text_trimmed()
    }

    pub fn validate(&self) -> Option<LuErr> {
        if let Err(e) = Pattern::new(&self.value()).map_err(|e| {
            AstErr::PatternError {
                pattern: self.to_item(),
                err: e.to_string(),
            }
            .into()
        }) {
            Some(e)
        } else {
            None
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
