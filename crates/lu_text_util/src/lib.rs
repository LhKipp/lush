use lu_error::LuResult;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SourceCode {
    pub text: String,
    pub path: PathBuf,
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
}

impl From<String> for SourceCode {
    fn from(s: String) -> Self {
        SourceCode::new_text(s)
    }
}
