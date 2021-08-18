#![allow(dead_code)]
pub mod ast;
mod build_tree;
mod syntax_node;

use lu_error::{LuErr, LuResult, ParseErr, ParseErrs};

pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub use lu_parser::{SyntaxKind, Token};
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};

pub use ast::{AstElement, AstElementChildren, AstNode, AstNodeChildren, AstToken};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
///
/// Currently the green node will always be a SourceFileNode
#[derive(Debug)]
pub struct Parse {
    green: GreenNode,
    errors: Vec<ParseErr>,
}

impl Parse {
    pub fn source_file(text: &str) -> Parse {
        let (green, errors) = build_tree::parse_text(text);

        // TODO add validation here
        // errors.extend(validation::validate(&root));

        Parse::new(green, errors)
    }

    fn new(green: GreenNode, errors: Vec<ParseErr>) -> Parse {
        Parse { green, errors }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn cast<T: AstNode>(&self) -> Option<T> {
        T::cast(self.syntax_node())
    }

    pub fn errors(&self) -> &[ParseErr] {
        &*self.errors
    }

    pub fn ok<T: AstNode>(self) -> LuResult<T> {
        if self.errors.is_empty() {
            Ok(self.cast::<T>().unwrap())
        } else {
            Err(LuErr::ParseErrs(ParseErrs::new(self.errors)))
        }
    }
}

#[test]
fn comp() {
    assert_eq!(1 + 1, 2);
    // Parse::source_file("echo hi");
}
