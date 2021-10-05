use std::fmt::Display;

use lu_error::SourceCodeItem;
use lu_syntax_elements::constants::USE_PATH_SEP;

// TODO how to represent paths within the same project?
pub struct UsePath {
    pub parts: Vec<String>,
    pub decl: SourceCodeItem,
}

impl UsePath {
    pub fn new(parts: Vec<String>, decl: SourceCodeItem) -> Self {
        UsePath { parts, decl }
    }
}

impl Display for UsePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(USE_PATH_SEP))
    }
}
