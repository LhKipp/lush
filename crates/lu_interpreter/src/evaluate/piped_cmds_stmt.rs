use lu_error::LuResult;
use lu_syntax::ast::PipedCmdsStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Interpreter, Variable};

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let mut previous_val = Value::Nil;
        for cmd in self.cmds() {
            state
                .scope
                .lock()
                .cur_mut_frame()
                .insert_var(Variable::new_in(previous_val));
            previous_val = cmd.evaluate(state)?
        }

        Ok(previous_val)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_syntax::ast::SourceFileNode;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/piped_cmds_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
