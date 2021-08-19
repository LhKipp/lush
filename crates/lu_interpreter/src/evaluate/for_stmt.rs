use contracts::ensures;
use log::debug;
use lu_error::LuResult;
use lu_syntax::{ast::ForStmtNode, AstToken};
use lu_value::Value;

use crate::{Evaluable, Interpreter, ScopeFrameTag};

impl Evaluable for ForStmtNode {
    #[ensures(&ret.is_ok() -> (ret == LuResult::Ok(Value::Nil)))]
    fn do_evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let stmts: Vec<_> = self.statements().collect();
        if stmts.is_empty() {
            debug!("Empty for stmt");
            // Empty for statement. This is a noop. Should have been a warning (at least).
            return Ok(Value::Nil);
        }
        let var_names: Vec<String> = self
            .var_names()
            .into_iter()
            .map(|decl| decl.text().to_string())
            .collect();
        assert!(var_names.len() > 0);
        let iterated_val = self.iterated_value().unwrap().value();

        // TODO iterate special over table
        match iterated_val {
            Value::Nil => todo!(),
            Value::Number(_) => todo!(),
            Value::BareWord(_) => todo!(),
            Value::String(str_to_iter) => {
                // TODO ret error
                assert_eq!(var_names.len(), 1);
                debug!("Iterating over string {} in for", str_to_iter);
                // Strings are iterated char wise
                for char in str_to_iter.chars() {
                    // We entered the for loop. We need to push a new scope and set the vars
                    {
                        let mut scope = state.scope.lock();
                        scope
                            .push_frame(ScopeFrameTag::ForStmtFrame)
                            .insert_var(var_names[0].clone(), Value::String(char.to_string()));
                    }
                    for stmt in &stmts {
                        stmt.evaluate(state)?;
                    }
                    {
                        let mut scope = state.scope.lock();
                        scope.pop_frame(ScopeFrameTag::ForStmtFrame);
                    }
                }
            }
            Value::Array(_arr) => todo!(),
        }

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

    #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/for_stmt/general.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResult<Value> {
        init_logger();
        let mut itprt = make_test_interpreter();

        itprt.evaluate_as::<SourceFileNode>(SourceCode::Text(s.to_string()))
    }
}
