use crate::action::dbg_action_prelude::*;

pub(crate) struct DbgNextAction {}

impl DbgAction for DbgNextAction {
    fn do_exec(
        &self,
        _: &str,
        stmt_id: &AstId,
        dbg_state: &mut DbgState,
        _: &mut SyScope,
    ) -> DbgActionResult {
        dbg_state.next_action_skip_after = Some(stmt_id.clone());
        DbgActionResult::StopDbgLoop
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
        "step over to the next statement (not recursing into function calls)"
    }
}

impl DbgActionAfterEval for DbgNextAction {
    fn exec_after_eval(&self, stmt_id: &AstId, dbg_state: &mut DbgState, _: &mut SyScope) {
        // Reset next_action_skip_after if we are at after_eval of the stmt
        if dbg_state.next_action_skip_after.as_ref() == Some(stmt_id) {
            dbg_state.next_action_skip_after = None
        }
    }
}

impl DbgNextAction {
    pub(crate) fn allows_dbg_intervention(dbg_state: &DbgState) -> bool {
        // Only allow dbg intervention if we are not skipping currently
        dbg_state.next_action_skip_after.is_none()
    }
}
