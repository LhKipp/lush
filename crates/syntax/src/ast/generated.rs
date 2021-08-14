#[rustfmt::skip]
pub(crate) mod nodes;
pub use nodes::*;

use crate::{AstNode, AstToken};

impl CmdStmtNode {
    pub fn possible_longest_name(&self) -> Vec<String> {
        self.syntax()
            .children_with_tokens()
            .take_while(|n| BareWordToken::can_cast(n.kind()))
            .filter_map(|n| n.into_token().map(BareWordToken::cast).flatten())
            .map(|n| n.text().to_string())
            .collect::<Vec<_>>()
    }
}
