use derive_more::Display;
use lu_text_util::{SourceCode, SourceCodeVariant};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use lu_error::SourceCodeItem;
use lu_syntax_elements::constants::MOD_PATH_FILE_SEP;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModPathVariant {
    StdPath,
    PluginPath,
    FilePath,
}

impl From<SourceCodeVariant> for ModPathVariant {
    fn from(v: SourceCodeVariant) -> Self {
        match v {
            SourceCodeVariant::StdCode => ModPathVariant::StdPath,
            SourceCodeVariant::PluginCode => ModPathVariant::PluginPath,
            SourceCodeVariant::FileCode => ModPathVariant::FilePath,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, new, Display)]
#[display(fmt = "{}/{:?}", mod_path, decl)]
pub struct UsePath {
    pub mod_path: ModPath,
    pub decl: SourceCodeItem,
}

// TODO how to represent paths within the same project?
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, new)]
pub struct ModPath {
    pub parts: Vec<String>,
    pub variant: ModPathVariant,
}

impl ModPath {
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
        )
    }

    pub fn as_f_path(&self) -> PathBuf {
        assert!(
            self.variant == ModPathVariant::PluginPath || self.variant == ModPathVariant::FilePath
        );
        self.parts.join("/").into()
    }

    pub fn from_src_code(src: &SourceCode, plugin_dir: &Path) -> Self {
        let normalized_path = src
            .path
            .strip_prefix(plugin_dir) // If src is a plugin, we remove the plugin_dir prefix (works better with use paths)
            .unwrap_or(src.path.as_ref());
        let parts = normalized_path
            .to_string_lossy()
            .split("/")
            .map(ToString::to_string)
            .collect();

        let variant = src.src_variant(plugin_dir).into();

        Self::new(parts, variant)
    }
}

impl Display for ModPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(MOD_PATH_FILE_SEP))
    }
}
