use log::debug;
// use log::debug;
use lu_error::LuResult;
use lu_interpreter::{Command, EvalArg, Interpreter, Variable};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct TestPrintCmd {}

impl Command for TestPrintCmd {
    fn name(&self) -> &str {
        "tprint"
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let mut l_scope = state.scope.lock();
        let args = self.expect_args(&l_scope).clone();
        let global_f = l_scope.global_mut_frame();

        if let Some(test_print_vars) = global_f.get_mut_var("t_printed") {
            let vals = test_print_vars.val.expect_array();
            let len = args.len();
            vals.extend((0..len).map(move |i| args[i].clone()))
        } else {
            debug!("Inserted t_printed");
            global_f.insert_var(Variable::new(
                "t_printed".to_string(),
                Value::Array(args.clone()),
            ));
        }
        Ok(Value::Nil)
    }
}
