use derive_more::Display;

use enum_as_inner::EnumAsInner;
use lu_syntax::{ast::UseStmtNode, AstNode};
use lu_text_util::{SourceCode, SourceCodeVariant};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use lu_error::SourceCodeItem;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, new, Display)]
#[display(fmt = "{}/{:?}", mod_path, decl)]
pub struct UsePath {
    pub mod_path: ModPath,
    pub decl: SourceCodeItem,
}

impl UsePath {
    pub fn from_node(use_stmt: &UseStmtNode) -> Self {
        UsePath {
            mod_path: ModPath::from_node(use_stmt),
            decl: use_stmt.to_item(),
        }
    }
}

// TODO how to represent paths within the same project?
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, new, EnumAsInner)]
pub enum ModPath {
    // PlugPath with plug-path elements
    PlugPath(PathBuf),
    StdPath(String),
    FilePath(PathBuf),
}

impl ModPath {
    pub fn from_node(use_path_node: &UseStmtNode) -> ModPath {
        if let Some(plug_path) = use_path_node.plugin_path() {
            if plug_path.is_std_path() {
                Self::StdPath(plug_path.to_string())
            } else {
                Self::PlugPath(plug_path.as_f_path())
            }
        } else if let Some(file_path) = use_path_node.file_path() {
            Self::FilePath(file_path.path())
        } else {
            unreachable!("UseStmt is either or")
        }
    }

    pub fn from_src_code(src: &SourceCode, plugin_dir: &Path) -> Self {
        match src.src_variant(plugin_dir) {
            SourceCodeVariant::StdCode => Self::StdPath(
                src.path
                    .to_string_lossy()
                    .split("/")
                    .map(ToString::to_string)
                    .collect(),
            ),
            SourceCodeVariant::PluginCode => Self::PlugPath(
                src.path
                    .strip_prefix(plugin_dir)
                    .unwrap()
                    .to_string_lossy()
                    .split("/")
                    .map(ToString::to_string)
                    .collect(),
            ),
            SourceCodeVariant::FileCode => Self::FilePath(src.path.clone()),
        }
    }
}

impl Display for ModPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModPath::StdPath(p) => {
                write!(f, "{}", p)
            }
            ModPath::FilePath(p) | ModPath::PlugPath(p) => {
                write!(f, "{}", p.display())
            }
        }
    }
}
