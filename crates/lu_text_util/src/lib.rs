#[macro_use]
extern crate educe;
extern crate derive_new;

use derive_new::new;
use lu_error::LuResult;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Educe, Eq, new)]
#[educe(PartialEq)]
pub struct SourceCode {
    #[educe(PartialEq(ignore))]
    pub text: String,
    pub path: PathBuf,
}

pub enum SourceCodeVariant {
    /// Lu code from the std library
    StdCode,
    /// Lu code from a plugin
    PluginCode,
    /// Lu code from any kind of file
    FileCode,
}

impl SourceCode {
    pub fn new_text(text: String) -> SourceCode {
        SourceCode {
            text,
            path: "__TMP_TEXT__".into(),
        }
    }

    pub fn new_file(f: PathBuf) -> LuResult<SourceCode> {
        Ok(SourceCode {
            text: lu_fs::read_to_string(f.as_path())?,
            path: f,
        })
    }

    // TODO this func is very convenient but crosses all boundaries, and knows much more, than it
    // actually should
    pub fn src_variant(&self, plugin_dir: &Path) -> SourceCodeVariant {
        const LU_STD_PATH_START: &str = "crates/lu_cmds/";
        if self.path.starts_with(LU_STD_PATH_START) {
            SourceCodeVariant::StdCode
        } else if self.path.starts_with(plugin_dir) {
            SourceCodeVariant::PluginCode
        } else {
            SourceCodeVariant::FileCode
        }
    }
}

impl From<String> for SourceCode {
    fn from(s: String) -> Self {
        SourceCode::new_text(s)
    }
}

/// New SourceCodeItem pointing to the file and line from the caller
#[macro_export]
macro_rules! lu_source_code {
    () => {{
        {
            let text = include_str!("array.rs");
            let path = file!();
            // TODO better source code item
            lu_text_util::SourceCode::new(text.to_string(), path.into())
        }
    }};
}
