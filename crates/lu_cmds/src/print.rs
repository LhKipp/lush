use lu_error::LuResult;
use lu_interpreter::{Command, EvalArg, Evaluable, Interpreter};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct PrintCmd {}

impl Command for PrintCmd {
    fn name(&self) -> &str {
        "print"
    }
}

impl Evaluable for PrintCmd {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let args = match &state.scope.lock().cur_frame().get_var("args").unwrap().val {
            Value::Array(vals) => vals[1..].to_vec(), // Always erase $arg.0 (cmd name)
            _ => unreachable!(),
        };
        Ok(Value::new_array(args))
    }
}
