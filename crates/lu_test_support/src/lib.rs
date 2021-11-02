mod playground;
pub mod test_prelude;

#[macro_use]
extern crate vec_rc;

use std::rc::Rc;

pub use playground::*;
use pretty_env_logger::env_logger;

use lu_cmds::PrintCmd;
use lu_interpreter::InterpreterCfg;
use lu_interpreter_structs::{Command, ScopeFrame, ScopeFrameTag, Value, VarDeclNode, Variable};

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
        make_global_frame(),
        InterpreterCfg {
            plugin_dir: playground.plugin_dir(),
        },
    )
}

fn make_global_frame() -> ScopeFrame<Variable> {
    let cmds: Vec<Rc<dyn Command>> = vec_rc![PrintCmd::new()];

    let mut frame = ScopeFrame::new(ScopeFrameTag::GlobalFrame);
    for cmd in cmds {
        frame.insert_var(Variable::new(
            cmd.name().to_string(),
            Value::new_func(cmd),
            VarDeclNode::Dummy,
        ));
    }
    frame
}
