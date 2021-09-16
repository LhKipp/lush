//! This module defines Concrete Syntax Tree (CST), used by rust-analyzer.
//!
//! The CST includes comments and whitespace, provides a single node type,
//! `SyntaxNode`, and a basic traversal API (parent, children, siblings).
//!
//! The *real* implementation is in the (language-agnostic) `rowan` crate, this
//! module just wraps its API.

use lu_error::ParseErr;
use rowan::{GreenNodeBuilder, Language, SmolStr};

use crate::SyntaxKind;

pub(crate) use rowan::GreenNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuLanguage {}
impl Language for LuLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<LuLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<LuLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<LuLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<LuLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<LuLanguage>;

#[derive(Default)]
pub struct SyntaxTreeBuilder {
    errors: Vec<ParseErr>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<ParseErr>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = LuLanguage::kind_to_raw(kind);
        self.inner.token(kind, SmolStr::new(text))
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = LuLanguage::kind_to_raw(kind);
        self.inner.start_node(kind)
    }

    pub fn finish_node(&mut self) {
        self.inner.finish_node()
    }

    pub fn error(&mut self, error: ParseErr) {
        self.errors.push(error)
    }
}
