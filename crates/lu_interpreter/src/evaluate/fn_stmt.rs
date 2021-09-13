use lu_error::LuResult;
use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

#[allow(unused_imports)]
use crate::{Callable, EvalArg, Evaluable, Function, Interpreter, Variable};

impl Evaluable for FnStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut Interpreter) -> LuResult<Value> {
        // let mut l_scope = state.scope.lock();

        // let name = self.name().unwrap_or("".to_string());
        // let sign: Signature = fn_node
        //     .signature()
        //     .map(|sig_n| sig_n.into())
        //     .or(Signature::new());
        // TODO create right signature from function
        // let sign = Signature::default();
        // let frame_id = l_scope.get_cur_frame_id();

        // let func = Function::new(name.clone(), self.clone(), frame_id);
        // let func: Callable = func.into();
        // l_scope
        //     .cur_mut_frame()
        //     .insert(name.clone(), Variable::new(name, Value::new_func(func)));

        Ok(Value::Nil)
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

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/fn_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
