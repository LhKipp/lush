use lu_error::LuResult;
use lu_syntax::ast::PipedCmdsStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, Variable};

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let mut previous_val = Value::Nil;
        for cmd in self.cmds() {
            state
                .scope
                .lock()
                .cur_mut_frame()
                .insert("in".to_string(), Variable::new_in(previous_val));
            previous_val = cmd.evaluate(state)?
        }

        Ok(previous_val)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/piped_cmds_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut evaluator = make_test_interpreter();

        evaluator
            .eval(s.to_string().into())
            .map_err(|errs| errs[0].clone())
    }
}
