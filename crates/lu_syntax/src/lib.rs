pub mod ast;
use lu_text_util::SourceCode;
mod build_tree;
mod syntax_node;
use lu_pipeline_stage::PipelineStage;

use build_tree::TreeBuilder;
use lu_error::{LuErr, LuResults};

use lu_parser::grammar::Rule;

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
    pub source: SourceCode,
    pub errors: Vec<LuErr>,
    green: GreenNode,
}

impl Parse {
    pub fn rule(code: SourceCode, rule: &dyn Rule) -> Self {
        let (green, errors) = TreeBuilder::build(&code.text, rule);
        let errors: Vec<LuErr> = errors.into_iter().map(|e| e.into()).collect();

        // TODO add validation here
        // errors.extend(validation::validate(&root));

        Parse::new(code, green, errors)
    }

    fn new(source: SourceCode, green: GreenNode, errors: Vec<LuErr>) -> Parse {
        Parse {
            source,
            green,
            errors,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn cast<T: AstNode>(&self) -> Option<T> {
        T::cast(self.syntax_node())
    }

    pub fn ok<T: AstNode>(self) -> LuResults<T> {
        if self.errors.is_empty() {
            Ok(self.cast::<T>().unwrap())
        } else {
            Err(self.errors)
        }
    }
}

impl PipelineStage for Parse {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        None
    }

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}
