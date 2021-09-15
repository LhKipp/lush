use lu_syntax_elements::BlockType;

/// When iterating over the ast its usefull to have additional data one can pass
/// to each visit. VisitArg is an helper enum containing generic args
#[derive(Clone, Debug)]
pub enum VisitArg {
    SourceFileName(String),
    BlockTypeArg(BlockType),
}
