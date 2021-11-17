#[macro_use]
extern crate manifest_dir_macros;
pub mod binary;
mod playground;
pub mod test_prelude;

use lu_error::lu_source_code_item;
pub use playground::*;
use pretty_env_logger::env_logger;

use lu_cmds::builtin;
use lu_interpreter::InterpreterCfg;
use lu_interpreter_structs::{ScopeFrame, ScopeFrameTag, Variable};
pub use temp_file::TempFile as TmpFile;

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

pub fn make_test_interpreter() -> (ScopeFrame<Variable>, InterpreterCfg) {
    make_test_interpreter_in_playground(Playground::new())
}

pub fn make_test_interpreter_in_playground(
    playground: Playground,
) -> (ScopeFrame<Variable>, InterpreterCfg) {
    (
        make_test_global_frame(playground.root().to_string_lossy().to_string()),
        InterpreterCfg {
            plugin_dir: playground.plugin_dir(),
        },
    )
}

fn make_test_global_frame(pwd: String) -> ScopeFrame<Variable> {
    let mut frame = ScopeFrame::new(ScopeFrameTag::GlobalFrame);
    // insert builtin cmds
    for cmd in builtin::all_builtin_cmds() {
        frame.insert_var(Variable::new_func(cmd));
    }
    frame.insert_var(Variable::new(
        "PWD".into(),
        pwd.clone().into(),
        lu_source_code_item!().into(),
    ));
    std::env::set_var("PWD", pwd);
    frame
}

pub fn make_tmp_file(text: &[u8]) -> TmpFile {
    temp_file::with_contents(text)
}
