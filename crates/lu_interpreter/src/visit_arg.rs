use std::path::PathBuf;

use crate::ScopeFrameTag;

/// When iterating over the ast its usefull to have additional data one can pass
/// to each visit. VisitArg is an helper enum containing generic args
#[derive(Clone, Debug)]
pub enum VisitArg {
    SourceFilePath(PathBuf),
    BlockTypeArg(ScopeFrameTag),
    SourceFileBlock(PathBuf),
}
