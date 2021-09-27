//! Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s
mod block_stmt;
mod cmd_stmt;
mod expr;
mod fn_stmt;
mod generated;
mod if_stmt;
mod let_stmt;
mod math_expr;
mod piped_cmds_stmt;
mod signature;
mod struct_stmt;
mod type_;
mod value_path_expr;
use std::marker::PhantomData;

use lu_error::SourceCodeItem;
use lu_parser::grammar::Rule;
use rowan::{SyntaxText, TextRange};

use crate::{
    syntax_node::{SyntaxNode, SyntaxNodeChildren, SyntaxToken},
    SyntaxElement, SyntaxElementChildren, SyntaxKind,
};

pub use self::generated::nodes::*;

pub trait HasRule {
    fn get_belonging_rule() -> Box<dyn Rule>;
}

pub trait HasSyntaxKind {
    fn get_syntax_kind(&self) -> SyntaxKind;
}

/// The main trait to go from untyped `SyntaxNode` to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn cast_element(syntax: SyntaxElement) -> Option<Self>
    where
        Self: Sized,
    {
        syntax.into_node().map(Self::cast).flatten()
    }

    fn syntax(&self) -> &SyntaxNode;
    //TODO check whether these methods are needed.
    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone()).unwrap()
    }
    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone()).unwrap()
    }

    fn to_item(&self) -> SourceCodeItem {
        SourceCodeItem::new(self.syntax().text_range().into(), self.syntax().text())
    }

    fn text_at(&self, range: &TextRange) -> SyntaxText {
        let idx_range = TextRange::up_to(range.end() - range.start());
        let idx_range = idx_range
            .checked_add(range.start() - self.syntax().text_range().start())
            .unwrap();
        self.syntax().text().slice(idx_range)
    }
}

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;
    fn text(&self) -> &str {
        self.syntax().text()
    }

    fn to_item(&self) -> SourceCodeItem {
        SourceCodeItem::new(self.syntax().text_range().into(), self.text().to_string())
    }
}

pub trait AstElement {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxElement) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> SyntaxElement;

    fn text(&self) -> String {
        match self.syntax() {
            rowan::NodeOrToken::Node(n) => n.text().to_string(),
            rowan::NodeOrToken::Token(t) => t.text().to_string(),
        }
    }

    fn to_item(&self) -> SourceCodeItem {
        SourceCodeItem::new(self.syntax().text_range().into(), self.text())
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstNodeChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

#[allow(dead_code)]
impl<N> AstNodeChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstNodeChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstNodeChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

/// An iterator over `SyntaxElement` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstElementChildren<N> {
    inner: SyntaxElementChildren,
    ph: PhantomData<N>,
}

impl<N> AstElementChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstElementChildren {
            inner: parent.children_with_tokens(),
            ph: PhantomData,
        }
    }
}

impl<N: AstElement> Iterator for AstElementChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

mod support {
    use crate::{AstElement, AstToken, SyntaxElement};

    use super::{AstElementChildren, AstNode, AstNodeChildren, SyntaxNode};

    #[allow(unused)]
    pub(super) fn node_child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub(super) fn token_child<N: AstToken>(parent: &SyntaxNode) -> Option<N> {
        parent
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(N::cast)
    }

    pub(super) fn element_child<N: AstElement>(parent: &SyntaxNode) -> Option<N> {
        parent.children_with_tokens().find_map(N::cast)
    }

    pub(super) fn token_children<N: AstToken>(parent: &SyntaxNode) -> Vec<N> {
        parent
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .filter_map(N::cast)
            .collect()
    }

    #[allow(dead_code)]
    pub(super) fn node_children<N: AstNode>(parent: &SyntaxNode) -> AstNodeChildren<N> {
        AstNodeChildren::new(parent)
    }

    pub(super) fn element_children<N: AstElement>(parent: &SyntaxNode) -> AstElementChildren<N> {
        AstElementChildren::new(parent)
    }
}
