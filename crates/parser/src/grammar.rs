//! This is the actual "grammar" of the lush language.
//!
//! Each function in this module and its children corresponds
//! to a production of the formal grammar. Submodules roughly
//! correspond to different *areas* of the grammar. By convention,
//! each submodule starts with `use super::*` import and exports
//! "public" productions via `pub(super)`.
//!
//! See docs for `Parser` to learn about API, available to the grammar,
//! and see docs for `Event` to learn how this actually manages to
//! produce parse trees.
//!
//! Code in this module also contains inline tests, which start with
//! `// test name-of-the-test` comment and look like this:
//!
//! ```
//! // test function_with_zero_parameters
//! // fn foo() {}
//! ```
//!
//! After adding a new inline-test, run `cargo test -p xtask` to
//! extract it as a standalone text-fixture into
//! `crates/syntax/test_data/parser/`, and run `cargo test` once to
//! create the "gold" value.
//!
//! Coding convention: rules like `where_clause` always produce either a
//! node or an error, rules like `opt_where_clause` may produce nothing.
//! Non-opt rules typically start with `assert!(p.at(FIRST_TOKEN))`, the
//! caller is responsible for branching on the first token.

mod cmd_stmt;
mod expr;
mod fn_stmt;

#[allow(unused_imports)]
use crate::{
    parser::{CompletedMarker, Marker, Parser, CMT_NL_WS},
    SyntaxKind::{self, *},
    TokenSet,
};

use self::{cmd_stmt::expect_cmd_stmt, fn_stmt::expect_fn_stmt};

pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    //SourceFile => statement % newline
    statements(p);
    m.complete(p, SourceFile);
}

fn statements(p: &mut Parser) {
    while p.next_non(CMT_NL_WS) != Eof {
        statement(p);
    }
}

fn block(p: &mut Parser) {
    while p.next_non(CMT_NL_WS) != End {
        statement(p);
    }
}

pub(crate) fn statement(p: &mut Parser) {
    match p.next_non(CMT_NL_WS) {
        Let => {
            // m.complete(p, LetStmt);
        }
        Fn => {
            expect_fn_stmt(p);
        }
        BareWord => {
            expect_cmd_stmt(p);
        }
        Eof => {}
        _ => {
            unreachable!("expected let, fn or cmd, found {:?}", p.current());
        }
    }
}

pub(crate) fn newline(p: &mut Parser) {
    if !p.eat(Newline) {
        p.error(format!("Expected a newline. Found {:?}", p.current()));
    }
}
