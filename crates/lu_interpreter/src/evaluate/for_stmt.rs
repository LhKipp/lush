use contracts::ensures;
use log::debug;
use lu_error::LuResult;
use lu_syntax::{ast::ForStmtNode, AstToken};
use lu_value::Value;

use crate::{variable::VarDeclNode, EvalArg, Evaluable, Evaluator, ScopeFrameTag, Variable};

impl Evaluable for ForStmtNode {
    #[ensures(&ret.is_ok() -> (ret == LuResult::Ok(Value::Nil)))]
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let block = self.block().unwrap();
        if block.is_empty() {
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
        // TODO iterate special over table
        match self.iterated_value().unwrap().value() {
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
                        scope.push_frame(ScopeFrameTag::ForStmtFrame).1.insert(
                            var_names[0].clone(),
                            Variable::new(
                                var_names[0].clone(),
                                Value::String(char.to_string()),
                                VarDeclNode::ForStmt(self.clone(), 0),
                            ),
                        );
                    }
                    block.evaluate(state)?;
                    {
                        let mut scope = state.scope.lock();
                        scope.pop_frame(&ScopeFrameTag::ForStmtFrame);
                    }
                }
            }
            Value::Array(_arr) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Function(_) => todo!(),
            Value::Strct(_) => todo!(),
        }

        Ok(Value::Nil)
    }
}
