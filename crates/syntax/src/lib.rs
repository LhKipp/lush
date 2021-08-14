#![allow(dead_code)]
pub mod ast;
mod build_tree;
mod syntax_error;
mod syntax_node;

use std::sync::Arc;

pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub use parser::{SyntaxKind, Token};
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};

pub use crate::syntax_error::SyntaxError;
pub use ast::{AstNode, AstToken};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
///
/// Currently the green node will always be a SourceFileNode
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parse {
    green: GreenNode,
    errors: Arc<Vec<SyntaxError>>,
}

impl Parse {
    fn source_file(text: &str) -> Parse {
        let (green, errors) = build_tree::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        // TODO add validation here
        // errors.extend(validation::validate(&root));

        assert_eq!(root.kind(), SyntaxKind::SourceFile);
        Parse {
            green,
            errors: Arc::new(errors),
        }
    }

    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse {
        Parse {
            green,
            errors: Arc::new(errors),
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn tree<T: AstNode>(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &*self.errors
    }

    pub fn ok<T: AstNode>(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

#[test]
fn comp() {
    assert_eq!(1 + 1, 2);
    // Parse::source_file("echo hi");
}
