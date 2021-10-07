use std::{fmt::Display, path::PathBuf};

use lu_error::SourceCodeItem;
use lu_syntax_elements::constants::USE_PATH_FILE_SEP;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UsePathVariant {
    StdPath,
    PluginPath,
    FilePath,
}

// TODO how to represent paths within the same project?
#[derive(Clone, Debug, Serialize, Deserialize, Educe, Eq)]
#[educe(PartialEq, Hash)]
pub struct UsePath {
    pub parts: Vec<String>,
    pub ty: UsePathVariant,
    #[educe(PartialEq(ignore), Hash(ignore))]
    pub decl: SourceCodeItem,
}

impl UsePath {
    pub fn new(parts: Vec<String>, ty: UsePathVariant, decl: SourceCodeItem) -> Self {
        UsePath { parts, decl, ty }
    }

    /// Pseudo path to the file with which the pipeline starts (main.lu / tmp_text ...)
    /// The path generated is faulty, but shouldn't hurt
    pub fn new_start_path(f_path: &PathBuf) -> UsePath {
        UsePath::new(
            f_path
                .to_string_lossy()
                .split("/")
                .map(ToString::to_string)
                .collect(),
            UsePathVariant::FilePath,
            SourceCodeItem::tmp_todo_item(),
        )
    }
}

impl Display for UsePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(USE_PATH_FILE_SEP))
    }
}
