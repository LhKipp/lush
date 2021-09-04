#[macro_use]
extern crate derive_new;

pub mod ast;
mod build_tree;
mod signature;
mod syntax_node;

use build_tree::TreeBuilder;
use lu_error::{LuErr, LuResult, ParseErr, ParseErrs};

use lu_parser::grammar::{Rule, SourceFileRule};

pub use signature::{
    ArgModifier, ArgSignature, FlagModifier, FlagSignature, Signature, VarArgSignature,
};
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
    pub fn rule(text: &str, rule: &dyn Rule) -> Self {
        let (green, errors) = TreeBuilder::build(text, rule);

        // TODO add validation here
        // errors.extend(validation::validate(&root));

        Parse::new(green, errors)
    }

    pub fn source_file(text: &str) -> Parse {
        Self::rule(text, &SourceFileRule {})
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
