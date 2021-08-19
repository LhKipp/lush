use log::debug;
// use log::debug;
use lu_error::LuResult;
use lu_interpreter::{Command, Interpreter};
use lu_value::Value;

#[derive(Debug)]
pub struct TestPrintCmd {}

impl Command for TestPrintCmd {
    fn name(&self) -> &str {
        "tprint"
    }

    fn do_run(&self, state: &mut Interpreter) -> LuResult<Value> {
        let args = match state.scope.lock().cur_frame().get_var("args").unwrap() {
            Value::Array(vals) => vals[1..].to_vec(), // Always erase $arg.0 (cmd name)
            _ => unreachable!(),
        };
        let mut scope = state.scope.lock();
        let global_f = scope.global_mut_frame();
        if let Some(test_print_vars) = global_f.get_mut_var("t_printed") {
            let vals = test_print_vars.expect_array();
            vals.extend(args)
        } else {
            debug!("Inserted t_printed");
            global_f.insert_var("t_printed".to_string(), Value::new_array(args));
        }
        Ok(Value::Nil)
    }
}
