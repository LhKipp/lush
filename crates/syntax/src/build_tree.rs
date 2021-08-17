#![allow(dead_code)]
#![allow(unused_imports)]
use std::mem;

// TODO remove dead code when all done
use lu_error::ParseErr;
use parser::Event;
use rowan::GreenNode;

use crate::{
    syntax_node::SyntaxTreeBuilder,
    SyntaxKind::{self, *},
    TextRange, TextSize, Token,
};

/// Bridges the parser with our specific syntax tree representation.
/// `TextTreeSink` also handles attachment of trivia (whitespace) to nodes.
pub(crate) struct TextTreeSink<'a> {
    text: &'a str,
    text_pos: TextSize,
    state: State,
    inner: SyntaxTreeBuilder,
}

#[derive(Eq, PartialEq)]
enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

impl<'a> TextTreeSink<'a> {
    fn token(&mut self, token: Token) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }
        self.do_token(token);
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        if self.state == State::PendingFinish {
            self.inner.finish_node();
        }
        self.inner.start_node(kind);
        self.state = State::Normal;
    }

    fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }
    }

    fn error(&mut self, error: ParseErr) {
        self.inner.error(error)
    }

    pub(super) fn new(text: &'a str) -> Self {
        Self {
            text,
            text_pos: 0.into(),
            state: State::PendingStart,
            inner: SyntaxTreeBuilder::default(),
        }
    }

    pub(super) fn finish(mut self) -> (GreenNode, Vec<ParseErr>) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingFinish => self.inner.finish_node(),
            State::PendingStart | State::Normal => unreachable!(),
        }

        self.inner.finish_raw()
    }

    fn do_token(&mut self, token: Token) {
        let range = TextRange::at(self.text_pos, token.len);
        let text = &self.text[range];
        self.text_pos += token.len;
        self.inner.token(token.kind, text);
    }
}

/// Parse the text as a source file
pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<ParseErr>) {
    let mut sink = TextTreeSink::new(text);
    let mut events = parser::parse(text);
    let mut forward_parents = Vec::new();
    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::tombstone()) {
            Event::Start {
                kind: Tombstone, ..
            } => (),

            Event::Start {
                kind,
                forward_parent,
            } => {
                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
                        Event::Start {
                            kind,
                            forward_parent,
                        } => {
                            if kind != Tombstone {
                                forward_parents.push(kind);
                            }
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            parser::Event::Finish => {}
            parser::Event::Token(token) => {
                sink.token(token);
            }
            parser::Event::Error(e) => sink.error(e),
        }
    }
    sink.finish()
}
