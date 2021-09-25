#[macro_use]
extern crate vec_box;

use pretty_env_logger::env_logger;

use lu_cmds::PrintCmd;
use lu_interpreter::{Callable, Command, Interpreter, Scope, ScopeFrameTag, VarDeclNode, Variable};
use lu_value::Value;

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

fn make_test_scope() -> Scope<Variable> {
    let cmds: Vec<Box<dyn Command>> = vec_box![PrintCmd::new()];

    let mut scope = Scope::new();
    let (_, frame) = scope.push_frame(ScopeFrameTag::GlobalFrame);
    for cmd in cmds {
        let cmd: Callable = cmd.into();
        frame.insert(
            cmd.name().to_string(),
            Variable::new(
                cmd.name().to_string(),
                Value::new_func(cmd),
                VarDeclNode::Dummy,
            ),
        )
    }
    scope
}

pub fn make_test_interpreter() -> Interpreter {
    Interpreter::new(make_test_scope())
}
