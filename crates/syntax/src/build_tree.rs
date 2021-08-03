#![allow(dead_code)]
#![allow(unused_imports)]
use std::mem;

// TODO remove dead code when all done
use parser::Event;
use rowan::GreenNode;

use crate::{
    SyntaxError,
    SyntaxKind::{self, *},
    TextRange,
    TextSize,
    // ast,
    Token,
};

// Bridges the parser with our specific syntax tree representation.
//
// `TextTreeSink` also handles attachment of trivia (whitespace) to nodes.
// pub(crate) struct TextTreeSink<'a> {
//     text: &'a str,
//     tokens: &'a [Token],
//     text_pos: TextSize,
//     token_pos: usize,
//     state: State,
//     inner: SyntaxTreeBuilder,
// }

// enum State {
//     PendingStart,
//     Normal,
//     PendingFinish,
// }

// impl<'a> TreeSink for TextTreeSink<'a> {
//     fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
//         match mem::replace(&mut self.state, State::Normal) {
//             State::PendingStart => unreachable!(),
//             State::PendingFinish => self.inner.finish_node(),
//             State::Normal => (),
//         }
//         self.eat_trivias();
//         let n_tokens = n_tokens as usize;
//         let len = self.tokens[self.token_pos..self.token_pos + n_tokens]
//             .iter()
//             .map(|it| it.len)
//             .sum::<TextSize>();
//         self.do_token(kind, len, n_tokens);
//     }

//     fn start_node(&mut self, kind: SyntaxKind) {
//         match mem::replace(&mut self.state, State::Normal) {
//             State::PendingStart => {
//                 self.inner.start_node(kind);
//                 // No need to attach trivias to previous node: there is no
//                 // previous node.
//                 return;
//             }
//             State::PendingFinish => self.inner.finish_node(),
//             State::Normal => (),
//         }

//         let n_trivias = self.tokens[self.token_pos..]
//             .iter()
//             .take_while(|it| it.kind.is_trivia())
//             .count();
//         let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
//         let mut trivia_end =
//             self.text_pos + leading_trivias.iter().map(|it| it.len).sum::<TextSize>();

//         let n_attached_trivias = {
//             let leading_trivias = leading_trivias.iter().rev().map(|it| {
//                 let next_end = trivia_end - it.len;
//                 let range = TextRange::new(next_end, trivia_end);
//                 trivia_end = next_end;
//                 (it.kind, &self.text[range])
//             });
//             n_attached_trivias(kind, leading_trivias)
//         };
//         self.eat_n_trivias(n_trivias - n_attached_trivias);
//         self.inner.start_node(kind);
//         self.eat_n_trivias(n_attached_trivias);
//     }

//     fn finish_node(&mut self) {
//         match mem::replace(&mut self.state, State::PendingFinish) {
//             State::PendingStart => unreachable!(),
//             State::PendingFinish => self.inner.finish_node(),
//             State::Normal => (),
//         }
//     }

//     fn error(&mut self, error: ParseError) {
//         self.inner.error(error, self.text_pos)
//     }
// }

// impl<'a> TextTreeSink<'a> {
//     pub(super) fn new(text: &'a str, tokens: &'a [Token]) -> Self {
//         Self {
//             text,
//             tokens,
//             text_pos: 0.into(),
//             token_pos: 0,
//             state: State::PendingStart,
//             inner: SyntaxTreeBuilder::default(),
//         }
//     }

//     pub(super) fn finish(mut self) -> (GreenNode, Vec<SyntaxError>) {
//         match mem::replace(&mut self.state, State::Normal) {
//             State::PendingFinish => {
//                 self.eat_trivias();
//                 self.inner.finish_node()
//             }
//             State::PendingStart | State::Normal => unreachable!(),
//         }

//         self.inner.finish_raw()
//     }

//     fn eat_trivias(&mut self) {
//         while let Some(&token) = self.tokens.get(self.token_pos) {
//             if !token.kind.is_trivia() {
//                 break;
//             }
//             self.do_token(token.kind, token.len, 1);
//         }
//     }

//     fn eat_n_trivias(&mut self, n: usize) {
//         for _ in 0..n {
//             let token = self.tokens[self.token_pos];
//             assert!(token.kind.is_trivia());
//             self.do_token(token.kind, token.len, 1);
//         }
//     }

//     fn do_token(&mut self, kind: SyntaxKind, len: TextSize, n_tokens: usize) {
//         let range = TextRange::at(self.text_pos, len);
//         let text = &self.text[range];
//         self.text_pos += len;
//         self.token_pos += n_tokens;
//         self.inner.token(kind, text);
//     }
// }

// pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
// let events = parser::parse(text);
// let mut forward_parents = Vec::new();
// for e in events {
//     match e {
//         parser::Event::Start {
//             kind,
//             forward_parent,
//         } => {
//             // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
//             // in the normal control flow, the parent-child relation: `A -> B -> C`,
//             // while with the magic forward_parent, it writes: `C <- B <- A`.

//             // append `A` into parents.
//             forward_parents.push(kind);
//             let mut idx = i;
//             let mut fp = forward_parent;
//             while let Some(fwd) = fp {
//                 idx += fwd as usize;
//                 // append `A`'s forward_parent `B`
//                 fp = match mem::replace(&mut events[idx], Event::tombstone()) {
//                     Event::Start {
//                         kind,
//                         forward_parent,
//                     } => {
//                         if kind != TOMBSTONE {
//                             forward_parents.push(kind);
//                         }
//                         forward_parent
//                     }
//                     _ => unreachable!(),
//                 };
//                 // append `B`'s forward_parent `C` in the next stage.
//             }

//             for kind in forward_parents.drain(..).rev() {
//                 sink.start_node(kind);
//             }
//         }
//         parser::Event::Finish => {}
//         parser::Event::Token { kind, n_raw_tokens } => {}
//         parser::Event::Error { msg } => {}
//     }
// }
// // let (tree, mut parser_errors) = tree_sink.finish();
// // parser_errors.extend(lexer_errors);

// // (tree, parser_errors)
// }
