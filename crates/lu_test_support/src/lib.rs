#[macro_use]
extern crate vec_box;

use pretty_env_logger::env_logger;

use lu_cmds::{PrintCmd, TestPrintCmd};
use lu_interpreter::{Callable, Command, Interpreter, ScopeFrameTag, Variable};
use lu_value::Value;

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

pub fn make_test_interpreter() -> Interpreter {
    let cmds: Vec<Box<dyn Command>> = vec_box![PrintCmd {}, TestPrintCmd {}];
    let state = Interpreter::new();
    {
        let mut l_scope = state.scope.lock();
        let (_, frame) = l_scope.push_frame(ScopeFrameTag::GlobalFrame);
        for cmd in cmds {
            let cmd: Callable = cmd.into();
            frame.insert(
                cmd.name().to_string(),
                Variable::new(cmd.name().to_string(), Value::new_func(cmd), None),
            )
        }
    }

    state
}
