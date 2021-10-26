use super::dbg_action_prelude::*;

pub(crate) struct DbgScopeAction {}

impl DbgAction for DbgScopeAction {
    fn long_name(&self) -> &'static str {
        "scope"
    }

    fn short_name(&self) -> &'static str {
        "sc"
    }
    fn args(&self) -> &[&'static str] {
        &[]
    }

    fn description(&self) -> &'static str {
        "Print the current scope"
    }

    fn do_exec(&self, args: &str, scope: &mut SyScope) -> DbgActionResult {
        if !args.is_empty() {
            println!("This command does not take arguments");
            return DbgActionResult::None;
        }

        println!("{:?}", scope.lock());
        DbgActionResult::None
    }
}
