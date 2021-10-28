pub(crate) mod dbg_action_prelude;
mod help;
mod next;
mod print;
mod scope;
mod skip;
mod step;
pub(crate) use help::DbgHelpAction;
use lu_interpreter_structs::{dbg_state::DbgState, SyScope};
use lu_syntax::AstId;
pub(crate) use next::DbgNextAction;
use once_cell::sync::Lazy;
pub(crate) use print::DbgPrintAction;
pub(crate) use scope::DbgScopeAction;
pub(crate) use skip::DbgSkipAction;
pub(crate) use step::DbgStepAction;
use vec_box::vec_box;

pub(crate) enum DbgActionResult {
    StopDbgLoop,
    None,
}

pub(crate) static ALL_DBG_ACTIONS: Lazy<Vec<Box<dyn DbgAction>>> = Lazy::new(|| {
    vec_box![
        DbgHelpAction {},
        DbgStepAction {},
        DbgNextAction {},
        DbgSkipAction {},
        DbgPrintAction {},
        DbgScopeAction {}
    ]
});

pub(crate) trait DbgAction: Sync + Send {
    fn matches(&self, line: &str) -> bool {
        if line == self.long_name() || line == self.short_name() {
            true
        } else if line.starts_with(self.long_name())
            && line.chars().skip(self.long_name().len()).next() == Some(' ')
        {
            true
        } else if line.starts_with(self.short_name())
            && line.chars().skip(self.short_name().len()).next() == Some(' ')
        {
            true
        } else {
            false
        }
    }

    fn do_exec(
        &self,
        args: &str,
        stmt_id: &AstId,
        dbg_state: &mut DbgState,
        scope: &mut SyScope,
    ) -> DbgActionResult;

    fn exec(
        &self,
        line: &str,
        stmt_id: &AstId,
        dbg_state: &mut DbgState,
        scope: &mut SyScope,
    ) -> DbgActionResult {
        assert!(self.matches(line));
        // We can't do
        // let line = line.strip_prefix(long_name);
        // let line = line.strip_prefix(short_name)
        // as that might strip args, it shouldn't
        let args = if line.starts_with(self.long_name()) {
            line.strip_prefix(self.long_name()).unwrap()
        } else if line.starts_with(self.short_name()) {
            line.strip_prefix(self.short_name()).unwrap()
        } else {
            unreachable!("Line maches")
        };
        self.do_exec(args, stmt_id, dbg_state, scope)
    }
    fn long_name(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn args(&self) -> &[&'static str];
    fn description(&self) -> &'static str;
}
