#[macro_use]
extern crate vec_box;

use pretty_env_logger::env_logger;

use std::collections::HashMap;

use lu_cmds::{PrintCmd, TestPrintCmd};
use lu_interpreter::{Command, Interpreter};

pub fn init_logger() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

pub fn make_test_interpreter() -> Interpreter {
    let cmds: Vec<Box<dyn Command>> = vec_box![PrintCmd {}, TestPrintCmd {}];
    let cmd_strg = cmds
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect::<HashMap<_, _>>()
        .into();

    Interpreter::new(cmd_strg)
}
