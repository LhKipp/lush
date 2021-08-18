use log::debug;
use lu_error::LuResult;
use lu_interpreter::{Command, Interpreter};
use lu_value::Value;

#[derive(Debug)]
pub struct PrintCmd {}

impl Command for PrintCmd {
    fn name(&self) -> &str {
        "print"
    }

    fn run(&self, state: &mut Interpreter) -> LuResult<Value> {
        let args = match state.scope.lock().cur_frame().get_var("args").unwrap() {
            Value::Array(vals) => vals[1..].to_vec(), // Always erase $arg.0 (cmd name)
            _ => unreachable!(),
        };
        debug!("{:?} returning {:?}", self.name(), args);
        Ok(Value::Array(args))
    }
}
