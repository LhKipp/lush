//! See [`Parser`].

use log::{debug, trace};
use std::cell::Cell;
use text_size::TextSize;

use lu_error::ParseErr;

use drop_bomb::DropBomb;

use crate::{
    event::Event,
    SyntaxKind::{self, Comment, Newline, Tombstone, *},
    Token, TokenSet, TokenSource,
};

pub const CMT_NL_WS: [SyntaxKind; 3] = [Comment, Newline, Whitespace];
pub const CMT_WS: [SyntaxKind; 2] = [Comment, Whitespace];

pub struct Parser {
    token_source: TokenSource,
    events: Vec<Event>,
    pub(crate) text_pos: TextSize,
    steps: Cell<u32>,
}

#[allow(unused)]
impl Parser {
    pub fn new(token_source: TokenSource) -> Parser {
        Parser {
            token_source,
            events: Vec::new(),
            steps: Cell::new(0),
            text_pos: 0.into(),
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
        if steps > 20000 {
            panic!("the parser seems stuck")
        }
        self.steps.set(steps + 1);

        let token = self.token_source[n].kind;
        token
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    /// next token not in ts
    pub(crate) fn next_non<TS: Into<TokenSet>>(&self, ts: TS) -> SyntaxKind {
        let ts: TokenSet = ts.into();
        self.token_source
            .iter()
            .map(|t| t.kind)
            .skip_while(|t| ts.contains(*t))
            .next()
            .unwrap_or(Eof)
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat<TS: Into<TokenSet>>(&mut self, kinds: TS) -> bool {
        let kinds: TokenSet = kinds.into();
        if !self.at(kinds) {
            trace!("Could not eat for ts {}", kinds);
            return false;
        }
        trace!("Eating {} (kinds was: {})", self.current(), kinds);
        //TODO is bump by 1 always correct?
        self.do_bump_cur();
        true
    }

    /// Consume the next token if kinds matches, but as a SyntaxKind of as_
    pub(crate) fn eat_as<TS: Into<TokenSet>>(&mut self, kinds: TS, as_: SyntaxKind) -> bool {
        let kinds: TokenSet = kinds.into();
        if !self.at(kinds) {
            trace!("Could not eat for ts {} as {}", kinds, as_);
            return false;
        }
        let cur = self.token_source.take_and_advance();
        trace!("Eating {:?} as {} (kinds was: {})", cur, as_, kinds);
        let new = Token::new(as_, cur.len);
        self.do_bump(new);
        true
    }

    /// Eat `orig` as `as_` delimited by `del`. If there can be multiple `del` between `orig`,
    /// multiple_del should be set to true, false otherwise
    pub(crate) fn eat_delimited_as<TS: Into<TokenSet> + Copy>(
        &mut self,
        orig: TS,
        as_: SyntaxKind,
        del: TS,
        multiple_del: bool,
    ) {
        let del: TokenSet = del.into();
        let orig: TokenSet = orig.into();

        trace!("eat_delimited_as {} {}, del: {}", orig, as_, del);
        while self.eat_as(orig, as_) {
            if multiple_del {
                while self.eat(del) {}
            } else {
                self.eat(del);
            }
        }
    }

    /// Consume the next token if `kind` matches.
    pub(crate) fn eat_while<TS: Into<TokenSet> + Copy>(&mut self, ts: TS) {
        let kinds: TokenSet = ts.into();
        trace!("Eating while {}", kinds);
        while self.at(kinds) {
            self.bump_any();
        }
    }

    /// Consume the next token until kind == current
    pub(crate) fn eat_until<TS: Into<TokenSet> + Copy>(&mut self, kinds: TS) {
        let kinds: TokenSet = kinds.into();
        trace!("Eating until {}", kinds);
        while !self.at(kinds) && !self.at(Eof) {
            self.bump_any();
        }
    }

    /// Eats `kinds` only if it comes after `after`. Leaves the token stream untouched otherwise
    pub(crate) fn eat_after<TS1: Into<TokenSet> + Copy, TS2: Into<TokenSet>>(
        &mut self,
        kinds: TS1,
        after: TS2,
    ) -> bool {
        let kinds = kinds.into();
        let after = after.into();
        if kinds.contains(self.next_non(after)) {
            self.eat_while(after);
            self.expect(kinds)
        } else {
            false
        }
    }

    /// Eats `kinds` only if it comes after `after`. Leaves the token stream untouched otherwise
    pub(crate) fn eat_after_as<TS1: Into<TokenSet> + Copy, TS2: Into<TokenSet> + Copy>(
        &mut self,
        kinds: TS1,
        as_: SyntaxKind,
        after: TS2,
    ) -> bool {
        let kinds = kinds.into();
        let after = after.into();
        if kinds.contains(self.next_non(after)) {
            self.eat_while(after);
            self.expect_as(kinds, as_)
        } else {
            false
        }
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect<TS: Into<TokenSet>>(&mut self, kinds: TS) -> bool {
        let kinds: TokenSet = kinds.into();
        if self.eat(kinds) {
            trace!("Expected {} and found one of them", kinds);
            return true;
        }
        trace!("Expected {}, but found none. Creating error.", kinds);
        self.error(format!("expected {}", kinds));
        false
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect_as<TS: Into<TokenSet>>(&mut self, kinds: TS, as_: SyntaxKind) -> bool {
        let kinds: TokenSet = kinds.into();
        if self.eat_as(kinds, as_) {
            trace!("Expected_as {} and found one of them", kinds);
            return true;
        }
        trace!("Expected_as {}, but found none. Creating error.", kinds);
        self.error(format!("expected {}", kinds).into());
        false
    }

    /// Expect `and_then` after `before`
    /// Example: p.expect_after(Newline, CMT_WS) // Expect a nl (with optional ws before)
    pub(crate) fn expect_after<TS1: Into<TokenSet> + Copy, TS2: Into<TokenSet>>(
        &mut self,
        kinds: TS1,
        after: TS2,
    ) -> bool {
        if !self.eat_after(kinds, after) {
            self.error(format!("expected {}", kinds.into()).into());
            false
        } else {
            true
        }
    }

    /// Eats `kinds` only if it comes after `after`. Leaves the token stream untouched otherwise
    pub(crate) fn expect_after_as<TS1: Into<TokenSet> + Copy, TS2: Into<TokenSet> + Copy>(
        &mut self,
        kinds: TS1,
        as_: SyntaxKind,
        after: TS2,
    ) -> bool {
        if !self.eat_after_as(kinds.clone(), as_, after) {
            self.error(format!("expected {}", kinds.into()).into());
            false
        } else {
            true
        }
    }

    /// Discards all token until `kinds`. Returns all discarded tokens
    pub(crate) fn discard_until<TS: Into<TokenSet> + Copy>(&mut self, kinds: TS) -> Vec<Token> {
        let mut discarded = Vec::new();
        let kinds: TokenSet = kinds.into();
        trace!("Discarding until {}", kinds);
        while !self.at(kinds) && !self.at(Eof) {
            discarded.push(self.token_source.take_and_advance())
        }
        discarded
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
        let ret = kinds.contains(self.current());
        trace!("Parser at {} is contained in {}", self.current(), kinds);
        ret
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
        self.do_bump_cur()
    }

    /// Emit error `err`
    pub(crate) fn error(&mut self, err: String) {
        debug!(
            "Parser error: {:?} (nth_tokens(0,1,2): {} {} {})",
            err,
            self.nth(0),
            self.nth(1),
            self.nth(2)
        );
        self.push_event(Event::Error(ParseErr::MessageAt(err, self.text_pos)));
    }
    pub(crate) fn do_bump_cur(&mut self) {
        let cur = self.token_source.take_and_advance();
        self.do_bump(cur);
    }

    pub(crate) fn do_bump(&mut self, token: Token) {
        trace!("Eating: {}", token.kind);
        self.text_pos += token.len;
        self.push_event(Event::Token(token));
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
    #[allow(unused)]
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

#[derive(Debug)]
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
