use std::fmt::Debug;

use lu_error::LuResult;
use value::Value;

use crate::Interpreter;

// pub struct CommandArgs {
//     /// The name by which the command has been called
//     pub cmd_call_name: Vec<String>,
//     /// The value from stdin
//     pub input: Value,
//     /// The arguments of the command
// }

// impl CommandArgs {
//     pub fn new() -> Self {
//         CommandArgs {
//             cmd_call_name: Vec::new(),
//             input: Value::Nil,
//         }
//     }
// }

pub trait Command: Send + Sync + Debug {
    fn name(&self) -> &str;
    fn run(&self, state: &mut Interpreter) -> LuResult<Value>;
}
