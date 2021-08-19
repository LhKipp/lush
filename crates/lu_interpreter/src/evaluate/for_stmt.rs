use contracts::ensures;
use lu_error::LuResult;
use lu_syntax::{ast::ForStmtNode, AstToken};
use lu_value::Value;

use crate::{Evaluable, Interpreter};

impl Evaluable for ForStmtNode {
    #[ensures(&ret.is_ok() -> (ret == LuResult::Ok(Value::Nil)))]
    fn evaluate(&self, state: &mut Interpreter) -> LuResult<Value> {
        let stmts: Vec<_> = self.statements().collect();
        if stmts.is_empty() {
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
                assert_eq!(var_names.len(), 1);
                // Strings are iterated char wise
                for char in str_to_iter.chars() {
                    // We entered the for loop. We need to push a new scope and set the vars
                    {
                        let mut scope = state.scope.lock();
                        scope.push_frame();
                        scope
                            .cur_mut_frame()
                            .insert_var(var_names[0].clone(), Value::String(char.to_string()));
                    }
                    for stmt in &stmts {
                        stmt.evaluate(state)?;
                    }
                    {
                        let mut scope = state.scope.lock();
                        scope.pop_frame();
                    }
                }
            }
            Value::Array(_arr) => todo!(),
        }

        Ok(Value::Nil)
    }
}
