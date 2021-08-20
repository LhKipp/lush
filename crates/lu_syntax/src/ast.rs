//! Abstract Syntax Tree, layered on top of untyped `SyntaxNode`s
mod expr;
mod generated;
mod let_stmt;
mod value_path_expr;
use std::marker::PhantomData;

use lu_parser::grammar::Rule;

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
}

pub trait AstElement {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxElement) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> SyntaxElement;
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstNodeChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

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

    pub(super) fn node_children<N: AstNode>(parent: &SyntaxNode) -> AstNodeChildren<N> {
        AstNodeChildren::new(parent)
    }

    pub(super) fn element_children<N: AstElement>(parent: &SyntaxNode) -> AstElementChildren<N> {
        AstElementChildren::new(parent)
    }
}
