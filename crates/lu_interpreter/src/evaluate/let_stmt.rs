use lu_error::LuResult;
use lu_syntax::ast::LetStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Interpreter, Variable};

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let var_name = self.var_name().unwrap();
        let val = self.value().unwrap();

        let val = val.evaluate(state)?;

        state
            .scope
            .lock()
            .cur_mut_frame()
            .insert_var(Variable::new(var_name, val));

        Ok(Value::Nil)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_syntax::ast::SourceFileNode;
    use lu_test_support::make_test_interpreter;
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/let_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        lu_test_support::init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
