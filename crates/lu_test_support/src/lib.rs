mod playground;
pub mod test_prelude;

#[macro_use]
extern crate vec_rc;

use std::rc::Rc;

pub use playground::*;
use pretty_env_logger::env_logger;

use lu_cmds::PrintCmd;
use lu_interpreter::{Interpreter, InterpreterCfg};
use lu_interpreter_structs::{Command, Scope, ScopeFrameTag, Value, VarDeclNode, Variable};

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

fn make_test_scope() -> Scope<Variable> {
    let cmds: Vec<Rc<dyn Command>> = vec_rc![PrintCmd::new()];

    let mut scope = Scope::new();
    let (_, frame) = scope.push_frame(ScopeFrameTag::GlobalFrame);
    for cmd in cmds {
        frame.insert_var(Variable::new(
            cmd.name().to_string(),
            Value::new_func(cmd),
            VarDeclNode::Dummy,
        ));
    }
    scope
}

// const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
// const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
//     let playground_dir: PathBuf = [MANIFEST, "crates", CRATE_NAME, "playground"]
//         .iter()
//         .collect();

pub fn make_test_interpreter() -> Interpreter {
    make_test_interpreter_in_playground(Playground::new())
}

pub fn make_test_interpreter_in_playground(playground: Playground) -> Interpreter {
    let config = InterpreterCfg {
        plugin_dir: playground.plugin_dir(),
    };
    Interpreter::new(make_test_scope(), config)
}
