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

mod block_stmt;
mod cmd_stmt;
mod comments;
mod expr;
mod file_name_expr;
mod fn_stmt;
mod for_stmt;
mod if_stmt;
mod incomplete_input;
mod let_stmt;
mod piped_cmds_stmt;
mod ret_stmt;
mod signature;
mod struct_stmt;
mod type_;
mod use_stmt;
mod value_path_expr;

use itertools::Itertools;
use log::debug;
use once_cell::unsync::Lazy;
use text_size::TextSize;
use vec_box::vec_box;

use crate::{
    parser::{CompletedMarker, Parser, CMT_NL_WS},
    token_set::TokenSet,
    SyntaxKind::{self, *},
    Token,
};

pub use block_stmt::BlockStmtRule;
pub use cmd_stmt::CmdStmtRule;
pub use expr::{ArrayExprRule, NumberExprRule, StringExprRule, TableExprRule, ValueExprRule};
pub use fn_stmt::FnStmtRule;
pub use for_stmt::ForStmtRule;
pub use if_stmt::IfElifElseRule;
pub use let_stmt::LetStmtRule;
pub use piped_cmds_stmt::PipedCmdsStmtRule;
pub use ret_stmt::RetStmtRule;
pub use signature::SignatureRule;
pub use struct_stmt::{StrctCtorExprRule, StrctStmtRule};
pub use type_::{ArrayTypeRule, FnTypeRule, LuTypeRule};
pub use use_stmt::UseStmtRule;
pub use value_path_expr::ValuePathExprRule;

pub trait Rule {
    /// Returns the name of the rule
    fn name(&self) -> String;
    /// Returns whether parser state matches this rule
    fn matches(&self, p: &mut Parser) -> bool;
    /// Internal function
    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker>;

    /// Expect this rule. If rule does not match, panic!
    fn expect(&self, p: &mut Parser) {
        debug!("Expecting {:?}", self.name());
        assert!(self.matches(p));
        self.parse_rule(p);
    }

    /// Only parse if this rule matches
    fn opt(&self, p: &mut Parser) -> Option<CompletedMarker> {
        debug!("Testing for optional {:?}", self.name());
        if self.matches(p) {
            self.parse_rule(p)
        } else {
            None
        }
    }

    /// Parse this rule. If it doesn't match a error event will be generated
    fn parse(&self, p: &mut Parser) -> Option<CompletedMarker> {
        debug!(
            "Parsing {} at token {:?}",
            self.name(),
            p.next_non(CMT_NL_WS)
        );
        let result = self.parse_rule(p);
        debug!(
            "Finished Parsing {}, Now at {:?}",
            self.name(),
            p.next_non(CMT_NL_WS)
        );
        result
    }
}

impl Rule for SyntaxKind {
    fn name(&self) -> String {
        format!("{:?}", self)
    }

    fn matches(&self, p: &mut Parser) -> bool {
        p.current() == *self
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        p.expect(*self);
        None
    }
}

pub(crate) struct OrRule {
    /// kind to specify this or Rule (if left empty, or1 | or2 | ... is used)
    kind: Option<String>,
    rules: Vec<Box<dyn Rule>>,
}

impl OrRule {
    fn fmt_names(&self) -> String {
        self.rules.iter().map(|rule| rule.name()).join(" | ")
    }
}

impl Rule for OrRule {
    fn name(&self) -> String {
        self.kind.clone().unwrap_or(self.fmt_names())
    }

    fn matches(&self, p: &mut Parser) -> bool {
        self.rules.iter().any(|rule| rule.matches(p))
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        if let Some(rule) = self.rules.iter().find(|rule| rule.matches(p)) {
            debug!("OrRule {}: Parsing rule {}", self.name(), rule.name());
            rule.parse(p)
        } else {
            p.error(format!(
                "Expected {}, but found {}",
                self.name(),
                // TODO can't realy tell whether ws has significance for any rule here...
                // Best bet is to take next_non(CMT_NL_WS)
                p.next_non(CMT_NL_WS)
            ));
            None
        }
    }
}

impl<T> Rule for Lazy<T>
where
    T: Rule,
{
    fn name(&self) -> String {
        format!("Lazy({})", (**self).name())
    }

    fn matches(&self, p: &mut Parser) -> bool {
        (**self).matches(p)
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        (**self).parse_rule(p)
    }
}

pub struct SourceFileRule {
    pub mark_as_cli_line: bool,
    pub offset: TextSize,
}

impl Default for SourceFileRule {
    fn default() -> Self {
        SourceFileRule {
            mark_as_cli_line: false,
            offset: 0.into(),
        }
    }
}
impl Rule for SourceFileRule {
    fn name(&self) -> String {
        "lu file".into()
    }

    fn parse_rule(&self, p: &mut Parser) -> Option<CompletedMarker> {
        let m = p.start();
        if self.mark_as_cli_line {
            p.do_bump(Token::new(CliLine, 0.into()));
        }
        if self.offset != 0.into() {
            p.do_bump(Token::new(Offset, self.offset));
        }
        BlockStmtRule::source_file_block().parse(p);
        Some(m.complete(p, SourceFile))
    }

    fn matches(&self, _: &mut Parser) -> bool {
        unreachable!("Should never be requested")
    }
}

fn second_level_stmt() -> OrRule {
    OrRule {
        kind: None,
        rules: vec_box![
            LetStmtRule {},
            BlockStmtRule::new_lazy_rule(),
            PipedCmdsStmtRule {},
            RetStmtRule {},
            ForStmtRule {},
            IfElifElseRule {},
        ],
    }
}

fn top_level_stmt() -> OrRule {
    let mut second_level_stmt = second_level_stmt();
    second_level_stmt.rules.push(Box::new(UseStmtRule {}));
    second_level_stmt.rules.push(Box::new(FnStmtRule {}));
    second_level_stmt.rules.push(Box::new(StrctStmtRule {}));
    second_level_stmt.kind = None;

    second_level_stmt
}
