use crate::{AstNode, AstToken};

use super::{support, BareWordToken, ValuePathExprNode};

impl ValuePathExprNode {
    /// Returns: $part1.part2.part3 => [part1, part2, part3]
    pub fn var_name_parts(&self) -> Vec<String> {
        support::token_children::<BareWordToken>(self.syntax())
            .into_iter()
            .map(|n| n.text().to_string())
            .collect()
    }
}
