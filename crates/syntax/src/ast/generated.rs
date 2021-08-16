#[rustfmt::skip]
pub(crate) mod nodes;
pub use nodes::*;

use crate::{AstElementChildren, AstNode, AstToken};

use super::support;

impl CmdStmtNode {
    /// Returns the longest possible name of the called command
    /// Example: `git add file1 -N file2` will return [git, add, file1] as file1 could be
    /// a part of the command name. We can't necessarily distinguish the cmd name from the args...
    pub fn possible_longest_cmd_call_name(&self) -> Vec<String> {
        self.syntax()
            .children_with_tokens()
            .take_while(|n| BareWordToken::can_cast(n.kind()))
            .filter_map(|n| n.into_token().map(BareWordToken::cast).flatten())
            .map(|n| n.text().to_string())
            .collect()
    }

    /// All arguments of this command. This does include the command name parts.
    pub fn args(&self) -> AstElementChildren<ExpressionNode> {
        support::element_children::<ExpressionNode>(self.syntax())
    }
}

impl ArrayExprNode {
    pub fn values(&self) -> AstElementChildren<ExpressionNode> {
        support::element_children::<ExpressionNode>(self.syntax())
    }
}

impl StringExprNode {
    pub fn text(&self) -> String {
        support::token_child::<StringContentToken>(self.syntax())
            .unwrap()
            .text()
            .to_string()
    }
}
