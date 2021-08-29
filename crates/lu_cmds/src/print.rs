use lu_error::LuResult;
use lu_interpreter::{Command, EvalArg, Interpreter};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct PrintCmd {}

impl Command for PrintCmd {
    fn name(&self) -> &str {
        "print"
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let l_scope = state.scope.lock();
        let args = self.expect_args(&l_scope);
        Ok(Value::Array(args.clone()))
    }
}
