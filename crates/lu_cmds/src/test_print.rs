use log::debug;
// use log::debug;
use lu_error::LuResult;
use lu_interpreter::{Command, EvalArg, Evaluable, Interpreter, Variable};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct TestPrintCmd {}

impl Command for TestPrintCmd {
    fn name(&self) -> &str {
        "tprint"
    }
}

impl Evaluable for TestPrintCmd {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let args = match &state.scope.lock().cur_frame().get_var("args").unwrap().val {
            Value::Array(vals) => vals[1..].to_vec(), // Always erase $arg.0 (cmd name)
            _ => unreachable!(),
        };
        let mut scope = state.scope.lock();
        let global_f = scope.global_mut_frame();
        if let Some(test_print_vars) = global_f.get_mut_var("t_printed") {
            let vals = test_print_vars.val.expect_array();
            vals.extend(args)
        } else {
            debug!("Inserted t_printed");
            global_f.insert_var(Variable::new(
                "t_printed".to_string(),
                Value::new_array(args),
            ));
        }
        Ok(Value::Nil)
    }
}
