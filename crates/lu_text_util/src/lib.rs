use lu_error::LuResult;
use lu_fs::read_to_string;
use std::path::PathBuf;

pub enum SourceCode {
    Text(String),
    File(PathBuf),
}

impl SourceCode {
    pub fn to_string(self) -> LuResult<String> {
        match self {
            SourceCode::Text(s) => Ok(s),
            SourceCode::File(p) => read_to_string(p),
        }
    }
}
