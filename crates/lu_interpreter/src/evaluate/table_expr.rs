use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::TableExprNode;

impl Evaluable for TableExprNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let strct_name = self.strct_name().unwrap();
        let field_names: Vec<_> = {
            let l_scope = scope.lock();
            let strct = l_scope.find_strct(strct_name.text()).unwrap();
            let l_strct = strct.read();
            l_strct
                .fields_sorted_by_order()
                .into_iter()
                .map(|field| field.name.clone())
                .collect()
        };

        let mut values = Vec::new();
        for row in self.rows() {
            let mut strct_vals = vec![];
            for (value, field) in row.values().zip(field_names.iter()) {
                strct_vals.push((field.clone(), value.evaluate(scope)?));
            }
            values.push(Value::new_strct(strct_name.to_string(), strct_vals));
        }
        Ok(Value::new_array(values))
    }
}
