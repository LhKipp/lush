use lu_error::LuResult;
use lu_fs::read_to_string;
use std::path::PathBuf;

pub enum SourceCode {
    Text(String),
    File(PathBuf),
}

impl SourceCode {
    /// Returns (Text, SourceName)
    pub fn unpack(self) -> LuResult<(String, Option<PathBuf>)> {
        match self {
            SourceCode::Text(s) => Ok((s, None)),
            SourceCode::File(p) => Ok((read_to_string(p.clone())?, Some(p))),
        }
    }

    pub fn to_string(self) -> LuResult<String> {
        match self {
            SourceCode::Text(t) => Ok(t),
            SourceCode::File(f) => read_to_string(f),
        }
    }
}
