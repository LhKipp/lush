use crate::action::dbg_action_prelude::*;

pub(crate) struct DbgNextAction {}

impl DbgAction for DbgNextAction {
    fn do_exec(&self, _: &str, _: &mut SyScope) -> DbgActionResult {
        todo!()
    }

    fn long_name(&self) -> &'static str {
        "next"
    }

    fn short_name(&self) -> &'static str {
        "n"
    }
    fn args(&self) -> &[&'static str] {
        &[]
    }

    fn description(&self) -> &'static str {
        "step over to the next evaluated statement (not recursing into function calls)"
    }
}
