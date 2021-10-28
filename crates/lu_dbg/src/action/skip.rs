use crate::action::dbg_action_prelude::*;

pub(crate) struct DbgSkipAction {}

impl DbgAction for DbgSkipAction {
    fn do_exec(&self, _: &str, _: &AstId, _: &mut DbgState, _: &mut SyScope) -> DbgActionResult {
        todo!()
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
        "step over to the next evaluated statement (not recursing into function calls)"
    }
}
