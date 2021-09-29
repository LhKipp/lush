use lu_error::SourceCodeItem;

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

    pub fn var_name_parts_with_item(&self) -> Vec<(String, SourceCodeItem)> {
        support::token_children::<BareWordToken>(self.syntax())
            .into_iter()
            .map(|n| (n.text().to_string(), n.to_item()))
            .collect()
    }
}
