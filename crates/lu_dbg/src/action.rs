pub(crate) mod dbg_action_prelude;
mod next;
mod print;
mod scope;
mod step;
use std::sync::Arc;

use lu_interpreter_structs::{Scope, Variable};
pub(crate) use next::DbgNextAction;
use parking_lot::Mutex;
pub(crate) use print::DbgPrintAction;
pub(crate) use scope::DbgScopeAction;
pub(crate) use step::DbgStepAction;

pub(crate) enum DbgActionResult {
    StopDbgLoop,
    None,
}

pub(crate) trait DbgAction {
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

    fn do_exec(&self, args: &str, scope: &mut Arc<Mutex<Scope<Variable>>>) -> DbgActionResult;
    fn exec(&self, line: &str, scope: &mut Arc<Mutex<Scope<Variable>>>) -> DbgActionResult {
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
        self.do_exec(args, scope)
    }
    fn long_name(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn args(&self) -> &[&'static str];
    fn description(&self) -> &'static str;
}
