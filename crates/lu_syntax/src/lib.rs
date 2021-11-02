pub mod ast;
mod build_tree;
mod syntax_node;
mod test;

use ast::{addr_of_sf_node, SourceFileNode};
use build_tree::TreeBuilder;
use derive_new::new;
use log::warn;
use lu_error::{util::Outcome, ParseErr, SourceCodeItem};
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
#[derive(Debug, new)]
pub struct Parse {
    pub source: SourceCode,
    pub sf_node: SourceFileNode,
}

impl Parse {
    pub fn source_file(source: SourceCode) -> Outcome<Parse> {
        let (green, errors) = TreeBuilder::build(&source.text);
        let sf_node = SyntaxNode::new_root(green);
        let sf_node = SourceFileNode::cast(sf_node)
            .expect("Only use this func, if your parsed a source file");
        let sf_node_addr = addr_of_sf_node(sf_node.syntax().clone());

        let errors = errors
            .into_iter()
            .map(|e| match e {
                ParseErr::MessageAt(msg, txt_pos) => ParseErr::MessageAtItem(
                    msg,
                    SourceCodeItem::new(
                        TextRange::new(txt_pos, txt_pos).into(),
                        "".to_string(),
                        sf_node_addr,
                    ),
                ),
                _ => {
                    warn!("Not mapping error and giving it a SourceCodeItem{:?}", e);
                    e
                }
            })
            .map(|e| e.into())
            .collect();

        // TODO add general ast validation here
        // errors.extend(validation::validate(&root));

        Outcome::new(Parse::new(source, sf_node), errors)
    }
}
