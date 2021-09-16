use lu_error::LuResult;
use lu_syntax::ast::LetStmtNode;
use lu_value::Value;

use crate::{variable::VarDeclNode, EvalArg, Evaluable, Evaluator, Variable};

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let var_name = self.var_name().unwrap();
        let val = self.value().unwrap();

        let val = val.evaluate(state)?;

        state.scope.lock().cur_mut_frame().insert(
            var_name.to_string(),
            Variable::new(var_name, val, Some(VarDeclNode::LetStmt(self.clone()))),
        );

        Ok(Value::Nil)
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResult;
    use lu_test_support::make_test_interpreter;
    use lu_value::Value;
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/let_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        lu_test_support::init_logger();
        let mut evaluator = make_test_interpreter();

        evaluator
            .eval(s.to_string().into())
            .map_err(|errs| errs[0].clone())
    }
}
