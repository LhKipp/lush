//! This module defines Concrete Syntax Tree (CST), used by rust-analyzer.
//!
//! The CST includes comments and whitespace, provides a single node type,
//! `SyntaxNode`, and a basic traversal API (parent, children, siblings).
//!
//! The *real* implementation is in the (language-agnostic) `rowan` crate, this
//! module just wraps its API.
#![allow(dead_code)] // TODO remove dead code when all done

use rowan::{GreenNodeBuilder, Language, SmolStr};

use crate::{Parse, SyntaxError, SyntaxKind, TextSize};

#[allow(unused_imports)]
pub(crate) use rowan::{GreenNode, GreenToken, NodeOrToken};

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
    errors: Vec<SyntaxError>,
    inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
        let green = self.inner.finish();
        (green, self.errors)
    }

    pub fn finish(self) -> Parse<SyntaxNode> {
        let (green, errors) = self.finish_raw();
        // if cfg!(debug_assertions) {
        //     let node = SyntaxNode::new_root(green.clone());
        //     crate::validation::validate_block_structure(&node);
        //     123
        // }
        Parse::new(green, errors)
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

    pub fn error(&mut self, error: parser::ParseError, text_pos: TextSize) {
        self.errors
            .push(SyntaxError::new_at_offset(error.error, text_pos))
    }
}
