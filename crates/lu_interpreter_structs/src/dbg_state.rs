use lu_syntax::AstId;

#[derive(Debug, Clone, Hash)]
pub struct DbgState {
    next_action_skip_after: Option<AstId>,
}
impl Default for DbgState {
    fn default() -> Self {
        DbgState {
            next_action_skip_after: None,
        }
    }
}
