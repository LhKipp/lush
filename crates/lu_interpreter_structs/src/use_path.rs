use std::{fmt::Display, path::PathBuf};

use lu_error::SourceCodeItem;
use lu_syntax_elements::constants::MOD_PATH_FILE_SEP;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModPathVariant {
    StdPath,
    PluginPath,
    FilePath,
}

// TODO how to represent paths within the same project?
#[derive(Clone, Debug, Serialize, Deserialize, Educe, Eq)]
#[educe(PartialEq, Hash)]
pub struct ModPath {
    pub parts: Vec<String>,
    pub variant: ModPathVariant,
    #[educe(PartialEq(ignore), Hash(ignore))]
    pub decl: SourceCodeItem,
}

impl ModPath {
    pub fn new(parts: Vec<String>, ty: ModPathVariant, decl: SourceCodeItem) -> Self {
        ModPath {
            parts,
            variant: ty,
            decl,
        }
    }

    /// Pseudo path to the file with which the pipeline starts (main.lu / tmp_text ...)
    /// The path generated is faulty, but shouldn't hurt
    pub fn new_start_path(f_path: &PathBuf) -> ModPath {
        ModPath::new(
            f_path
                .to_string_lossy()
                .split("/")
                .map(ToString::to_string)
                .collect(),
            ModPathVariant::FilePath,
            SourceCodeItem::tmp_todo_item(),
        )
    }

    pub fn as_f_path(&self) -> PathBuf {
        assert!(
            self.variant == ModPathVariant::PluginPath || self.variant == ModPathVariant::FilePath
        );
        self.parts.join("/").into()
    }
}

impl Display for ModPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(MOD_PATH_FILE_SEP))
    }
}
