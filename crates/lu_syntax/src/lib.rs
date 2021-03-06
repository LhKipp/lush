pub mod ast;
mod build_tree;
mod syntax_node;
mod test;
mod validate;

use ast::{addr_of_node, SourceFileNode};
use build_tree::TreeBuilder;
use derive_new::new;
use log::{debug, warn};
use lu_error::{util::Outcome, ParseErr, SourceCodeItem};
use lu_parser::{grammar::Rule, SourceFileRule};
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
    pub fn cli_line(line: SourceCode, offset: TextSize) -> Outcome<Parse> {
        Self::parse(
            line,
            &SourceFileRule {
                mark_as_cli_line: true,
                offset,
            },
        )
    }
    pub fn source_file(source: SourceCode) -> Outcome<Parse> {
        Self::parse(
            source,
            &SourceFileRule {
                mark_as_cli_line: false,
                offset: 0.into(),
            },
        )
    }

    fn parse(source: SourceCode, rule: &SourceFileRule) -> Outcome<Parse> {
        let (green, errors) = TreeBuilder::build(&source.text, rule);
        let sf_node = SyntaxNode::new_root(green);
        let sf_node = SourceFileNode::cast(sf_node).unwrap();
        let sf_node_addr = addr_of_node(sf_node.syntax().clone());

        let mut errors: Vec<_> = errors
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

        // General validation
        if let Err(ast_errs) = validate::validate(&sf_node) {
            errors.extend(ast_errs);
        }

        debug!(
            "Result of build_tree: {:#?}\nWith {} errors",
            sf_node,
            errors.len()
        );
        Outcome::new(Parse::new(source, sf_node), errors)
    }
}
