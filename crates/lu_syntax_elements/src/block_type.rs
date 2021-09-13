#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    SourceFileBlock,
    FnBlock,
    ForBlock,
}
