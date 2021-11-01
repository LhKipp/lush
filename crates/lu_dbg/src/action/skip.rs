use lu_parser::grammar::SourceFileRule;
use lu_syntax::Parse;

use crate::action::dbg_action_prelude::*;

pub(crate) struct DbgSkipAction {}

impl DbgAction for DbgSkipAction {
    fn do_exec(&self, arg: &str, _: &AstId, _: &mut DbgState, _: &mut SyScope) -> DbgActionResult {
        if arg.is_empty() {
            return DbgActionResult::StopDbgLoopAndContinueAsIfStmtRetsNil;
        }
        // TODO use ValueExprRule
        match Parse::rule(arg.into(), &SourceFileRule {}).as_results() {
            Err(errs) => {
                println!("Error parsing {} as a lu-value", arg);
                for err in errs {
                    // TODO nice display
                    println!("{:?}", err)
                }
                DbgActionResult::None
            }
            Ok(p) => DbgActionResult::StopDbgLoopAndContinueAsIfRetStmt(p),
        }
    }

    fn long_name(&self) -> &'static str {
        "skip"
    }

    fn short_name(&self) -> &'static str {
        "sk"
    }
    fn args(&self) -> &[&'static str] {
        &["[Value]"]
    }

    fn description(&self) -> &'static str {
        r#"Skip the next statement and continue as if the statement returned [Value].
Providing no value will return Value::Nil"#
    }
}
