use lu_syntax::ast::{
    ArrayExprNode, BareWordToken, BooleanExprNode, CmdOrValueExprElement, NumberExprNode,
    StringExprNode, TableExprNode, ValueExprElement, ValuePathExprNode,
};

use crate::evaluate::eval_prelude::*;

impl Evaluable for ValueExprElement {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[]
    }

    fn do_evaluate(&self, args: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            ValueExprElement::BooleanExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::BareWord(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::NumberExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::MathExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::StringExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::ValuePathExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::ArrayExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::TableExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::StrctCtorExpr(n) => n.evaluate_with_args(args, scope),
            ValueExprElement::CmdStmt(n) => n.evaluate_with_args(args, scope),
        }
    }
}

impl Evaluable for CmdOrValueExprElement {
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            CmdOrValueExprElement::CmdStmt(n) => n.evaluate_with_args(args, scope),
            CmdOrValueExprElement::ValueExpr(n) => n.evaluate_with_args(args, scope),
        }
    }
}

impl Evaluable for BareWordToken {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut SyScope) -> EvalResult {
        Ok(Value::BareWord(self.text().to_string()))
    }
}

impl Evaluable for NumberExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut SyScope) -> EvalResult {
        Ok(self.into())
    }
}

impl Evaluable for StringExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut SyScope) -> EvalResult {
        Ok(Value::String(self.text().to_string()))
    }
}

impl Evaluable for ValuePathExprNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let name_parts = self.var_name_parts();
        assert!(!name_parts.is_empty());
        let l_scope = scope.lock();
        let mut prev_var = l_scope
            .find_var(&name_parts[0])
            .expect("var always found")
            .val
            .clone();

        for index_name in &name_parts[1..] {
            // Its field indexing into a struct
            let (_, strct_fields) = prev_var.as_strct().expect("Prev var must be strct");
            prev_var = strct_fields
                .iter()
                .find_map(|(field_name, val)| {
                    if field_name == index_name {
                        Some(val.clone())
                    } else {
                        None
                    }
                })
                .expect("Index always works");
        }

        Ok(prev_var)
    }
}

impl Evaluable for ArrayExprNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let mut values = Vec::new();
        for val in self.values() {
            values.push(val.evaluate(scope)?);
        }
        Ok(Value::new_array(values))
    }
}

impl Evaluable for TableExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut SyScope) -> EvalResult {
        todo!()
    }
}

impl Evaluable for BooleanExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut SyScope) -> EvalResult {
        Ok(self.value().into())
    }
}
