use log::debug;
// use log::debug;
use lu_error::LuResult;
use lu_interpreter::{Command, EvalArg, Evaluator, Variable};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct TestPrintCmd {}

impl Command for TestPrintCmd {
    fn name(&self) -> &str {
        "tprint"
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let mut l_scope = state.scope.lock();
        let args = self.expect_args(&l_scope).clone();
        let global_f = l_scope.global_mut_frame();

        let var = "t_printed".to_string();
        if let Some(test_print_vars) = global_f.get_mut(&var) {
            let vals = test_print_vars.val.expect_array();
            let len = args.len();
            vals.extend((0..len).map(move |i| args[i].clone()))
        } else {
            debug!("Inserted t_printed");
            global_f.insert(
                var.clone(),
                Variable::new(var.clone(), Value::Array(args.clone()), None),
            );
        }
        Ok(Value::Nil)
    }
}
