//! See [`Parser`].

use log::debug;
use std::cell::Cell;

use drop_bomb::DropBomb;

#[allow(unused)]
#[macro_use]
use crate::T;

use crate::{
    event::Event,
    ParseError,
    SyntaxKind::{self, Comment, Error, Newline, Tombstone, *},
    Token, TokenSet, TokenSource,
};

pub(crate) const CMT_NL_WS: TokenSet = TokenSet::new(&[Comment, Newline, Whitespace]);

pub struct Parser {
    token_source: TokenSource,
    events: Vec<Event>,
    steps: Cell<u32>,
}

#[allow(unused)]
impl Parser {
    pub fn new(token_source: TokenSource) -> Parser {
        Parser {
            token_source,
            events: Vec::new(),
            steps: Cell::new(0),
        }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `Eof` kind is returned.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn current_token(&self) -> &Token {
        &self.token_source[0]
    }

    pub(crate) fn next(&self) -> SyntaxKind {
        self.nth(1)
    }

    /// Lookahead operation: returns the kind of the next nth
    /// token.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n <= 3);

        let steps = self.steps.get();
        assert!(steps <= 10_000_000, "the parser seems stuck");
        self.steps.set(steps + 1);

        let token = self.token_source[n].kind;
        debug!("token[{}]: {:?}", n, token);
        token
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    /// next token not in ts
    pub(crate) fn next_non<TS: Into<TokenSet>>(&self, ts: TS) -> SyntaxKind {
        let ts = ts.into();
        self.token_source
            .iter()
            .map(|t| t.kind)
            .skip_while(|t| ts.contains(*t))
            .next()
            .unwrap_or(Eof)
    }

    // /// Checks if all tokens until are kind are skippable. Expects kind to be present
    // pub(crate) fn is_skippable_until(&self, kind: SyntaxKind) -> bool {
    //     self.next_non_skippable() == kind
    // }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat<TS: Into<TokenSet>>(&mut self, kinds: TS) -> bool {
        if !self.at(kinds) {
            return false;
        }
        //TODO is bump by 1 always correct?
        self.do_bump();
        true
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat_while<TS: Into<TokenSet> + Copy>(&mut self, ts: TS) {
        while self.at(ts) {
            self.bump_any();
        }
    }

    /// Consume the next token until kind == current
    pub(crate) fn eat_until<TS: Into<TokenSet> + Copy>(&mut self, kinds: TS) {
        while !self.at(kinds) {
            self.bump_any();
        }
    }

    pub(crate) fn eat_empty_or_cmt_line(&mut self) -> bool {
        if self
            .token_source
            .iter()
            .take_while(|t| t.kind != Newline)
            .all(|t| t.kind == Comment || t.kind == Whitespace)
        {
            self.eat_until(Newline);
            true
        } else {
            false
        }
    }

    /// Checks if the current token is in `kinds`.
    pub(crate) fn at<TS: Into<TokenSet>>(&self, kinds: TS) -> bool {
        let kinds: TokenSet = kinds.into();
        kinds.contains(self.current())
    }

    // /// Checks if the current token is contextual keyword with text `t`.
    // pub(crate) fn at_contextual_kw(&self, kw: &str) -> bool {
    //     self.token_source.is_keyword(kw)
    // }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Advances the parser by one token
    pub(crate) fn bump_any(&mut self) {
        self.do_bump()
    }

    /// Emit error with the `message`
    /// FIXME: this should be much more fancy and support
    /// structured errors with spans and notes, like rustc
    /// does.
    pub(crate) fn error<T: Into<String>>(&mut self, message: T) {
        let msg = ParseError::new(message.into());
        self.push_event(Event::Error { msg })
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
        false
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_and_bump(&mut self, message: &str) {
        // match self.current() {
        // L_DOLLAR | R_DOLLAR => {
        //     let m = self.start();
        //     self.error(message);
        //     self.bump_any();
        //     m.complete(self, Error);
        // }
        // _ => {
        self.err_recover(message, TokenSet::EMPTY);
        // }
        // }
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_recover(&mut self, message: &str, recovery: TokenSet) {
        match self.current() {
            T!["{"] | T!["}"] => {
                self.error(message);
                return;
            }
            _ => (),
        }

        if self.at(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, Error);
    }

    fn do_bump(&mut self) {
        let cur_token = self.token_source.take_and_advance();
        debug!("Eating: {:?}", cur_token.kind);
        self.push_event(Event::Token(cur_token));
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }
}

/// See `Parser::start`.
pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

#[allow(unused)]
impl Marker {
    fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        let finish_pos = p.events.len() as u32;
        p.push_event(Event::Finish);
        CompletedMarker::new(self.pos, finish_pos, kind)
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start {
                    kind: Tombstone,
                    forward_parent: None,
                }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub struct CompletedMarker {
    start_pos: u32,
    finish_pos: u32,
    kind: SyntaxKind,
}

#[allow(unused)]
impl CompletedMarker {
    fn new(start_pos: u32, finish_pos: u32, kind: SyntaxKind) -> Self {
        CompletedMarker {
            start_pos,
            finish_pos,
            kind,
        }
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about `forward_parent` in `Event::Start`.
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into forward_parent(=2 in this case);
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
        let new_pos = p.start();
        let idx = self.start_pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_pos.pos - self.start_pos);
            }
            _ => unreachable!(),
        }
        new_pos
    }

    /// Undo this completion and turns into a `Marker`
    pub(crate) fn undo_completion(self, p: &mut Parser) -> Marker {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;
        match &mut p.events[start_idx] {
            Event::Start {
                kind,
                forward_parent: None,
            } => *kind = Tombstone,
            _ => unreachable!(),
        }
        match &mut p.events[finish_idx] {
            slot @ Event::Finish => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos)
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
