#[macro_use]
extern crate vec_box;

use pretty_env_logger::env_logger;

use std::collections::HashMap;

use interpreter::{Command, Interpreter};
use lu_cmds::PrintCmd;
use lu_error::LuResult;
use lu_text_util::SourceCode;
use value::Value;
use {conformance, serde_json};

#[conformance::tests(exact, serde=serde_json, file="test_data/general.json_test")]
fn general_interpreter_tests(s: &str) -> LuResult<Value> {
    let _ = env_logger::builder().is_test(true).try_init();

    let cmds: Vec<Box<dyn Command>> = vec_box![PrintCmd {}];
    let cmd_strg = cmds
        .into_iter()
        .map(|cmd| (cmd.name().to_string(), cmd))
        .collect::<HashMap<_, _>>()
        .into();

    let mut interpreter = Interpreter::new(cmd_strg);
    interpreter.evaluate(SourceCode::Text(s.to_string()))
}
