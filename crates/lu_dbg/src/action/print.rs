use super::dbg_action_prelude::*;

pub(crate) struct DbgPrintAction {}

impl DbgAction for DbgPrintAction {
    fn do_exec(
        &self,
        args: &str,
        _: &AstId,
        _: &mut DbgState,
        scope: &mut SyScope,
    ) -> DbgActionResult {
        let args = args.split_whitespace();
        let l_scope = scope.lock();
        for arg in args {
            let var = l_scope.find_var(arg);
            if let Some(var) = var {
                println!("{}: {}", var.name, var.val);
            } else {
                println!("{}: not found", arg);
            }
        }
        DbgActionResult::None
    }

    fn long_name(&self) -> &'static str {
        "print"
    }

    fn short_name(&self) -> &'static str {
        "p"
    }

    fn args(&self) -> &[&'static str] {
        &["[...var_names]"]
    }

    fn description(&self) -> &'static str {
        "Print variables specified by `...var_names`"
    }
}
