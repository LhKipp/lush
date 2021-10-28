use crate::action::{dbg_action_prelude::*, ALL_DBG_ACTIONS};

pub(crate) struct DbgHelpAction {}

impl DbgAction for DbgHelpAction {
    fn do_exec(&self, _: &str, _: &mut SyScope) -> DbgActionResult {
        println!(r#"Commands:"#);
        for cmd in &*ALL_DBG_ACTIONS {
            let args = if !cmd.args().is_empty() {
                format!(", {}", cmd.args().join(" "))
            } else {
                "".to_string()
            };
            println!(
                "  {}, {}{} - {}",
                cmd.long_name(),
                cmd.short_name(),
                args,
                cmd.description()
            );
        }

        DbgActionResult::None
    }

    fn long_name(&self) -> &'static str {
        "help"
    }

    fn short_name(&self) -> &'static str {
        "h"
    }
    fn args(&self) -> &[&'static str] {
        &[]
    }

    fn description(&self) -> &'static str {
        "show help"
    }
}
