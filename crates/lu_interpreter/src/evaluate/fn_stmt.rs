use lu_error::LuResult;
use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

use crate::Evaluator;
#[allow(unused_imports)]
use crate::{Callable, EvalArg, Evaluable, Function, Interpreter, Variable};

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut Evaluator) -> LuResult<Value> {
        Ok(Value::Nil)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResults;
    use lu_test_support::{init_logger, make_test_interpreter};
    use lu_text_util::SourceCode;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/fn_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResults<Value> {
        init_logger();
        let mut itprtr = make_test_interpreter();

        itprtr.run(SourceCode::Text(s.to_string()))
    }
}
