use super::dbg_action_prelude::*;

pub(crate) struct DbgStepAction {}

impl DbgAction for DbgStepAction {
    fn long_name(&self) -> &'static str {
        "step"
    }

    fn short_name(&self) -> &'static str {
        "s"
    }
    fn args(&self) -> &[&'static str] {
        &[]
    }

    fn description(&self) -> &'static str {
        "step to the next evaluated statement"
    }

    fn do_exec(&self, _: &str, _: &mut Arc<Mutex<Scope<Variable>>>) -> DbgActionResult {
        DbgActionResult::StopDbgLoop
    }
}
