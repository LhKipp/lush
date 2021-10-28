pub mod ast;
mod build_tree;
mod syntax_node;
mod test;

use ast::SourceFileNode;
use build_tree::TreeBuilder;
use derive_new::new;
use lu_error::{util::Outcome, LuErr};
use lu_parser::grammar::Rule;
use lu_text_util::SourceCode;

pub use ast::{AstElement, AstElementChildren, AstId, AstNode, AstNodeChildren, AstToken};
pub use syntax_node::{
    SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub use lu_parser::{SyntaxKind, Token};
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
///
/// Currently the green node will always be a SourceFileNode
#[derive(Debug, new)]
pub struct Parse {
    pub source: SourceCode,
    green: GreenNode,
}

impl Parse {
    pub fn rule(code: SourceCode, rule: &dyn Rule) -> Outcome<Self> {
        let (green, errors) = TreeBuilder::build(&code.text, rule);
        let errors: Vec<LuErr> = errors.into_iter().map(|e| e.into()).collect();

        // TODO add validation here
        // errors.extend(validation::validate(&root));

        Outcome::new(Parse::new(code, green), errors)
    }

    pub fn source_file_node(&self) -> SourceFileNode {
        self.cast::<SourceFileNode>()
            .expect("Only use this func, if your parsed a source file")
    }
    pub fn is_sf_parse(&self) -> bool {
        self.cast::<SourceFileNode>().is_some()
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn cast<T: AstNode>(&self) -> Option<T> {
        T::cast(self.syntax_node())
    }
}
