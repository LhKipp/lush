use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::StrctCtorExprNode;

impl Evaluable for StrctCtorExprNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let mut values = vec![];
        for field in self.fields() {
            let name = field.field_name();
            let val = field.value().unwrap().evaluate(state)?;

            values.push((name, val))
        }

        Ok(Value::new_strct(self.name(), values))
    }
}
