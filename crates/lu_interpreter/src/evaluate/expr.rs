use lu_syntax::ast::{
    ArrayExprNode, BareWordToken, CmdOrValueExprElement, NumberExprNode, StringExprNode,
    TableExprNode, ValueExprElement, ValuePathExprNode,
};

use crate::evaluate::eval_prelude::*;

impl Evaluable for ValueExprElement {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        match self {
            ValueExprElement::BareWord(n) => n.evaluate(scope),
            ValueExprElement::NumberExpr(n) => n.evaluate(scope),
            ValueExprElement::MathExpr(n) => n.evaluate(scope),
            ValueExprElement::StringExpr(n) => n.evaluate(scope),
            ValueExprElement::ValuePathExpr(n) => n.evaluate(scope),
            ValueExprElement::ArrayExpr(n) => n.evaluate(scope),
            ValueExprElement::TableExpr(n) => n.evaluate(scope),
            ValueExprElement::StrctCtorExpr(_) => todo!(),
        }
    }
}

impl Evaluable for CmdOrValueExprElement {
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        match self {
            CmdOrValueExprElement::CmdStmt(n) => n.evaluate_with_args(args, scope),
            CmdOrValueExprElement::PipedCmdsStmt(n) => n.evaluate_with_args(args, scope),
            CmdOrValueExprElement::ValueExpr(n) => n.evaluate_with_args(args, scope),
        }
    }
}

impl Evaluable for BareWordToken {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        Ok(Value::BareWord(self.text().to_string()))
    }
}

impl Evaluable for NumberExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        Ok(self.value())
    }
}

impl Evaluable for StringExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        Ok(Value::String(self.text().to_string()))
    }
}

impl Evaluable for ValuePathExprNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let name_parts = self.var_name_parts();
        assert_eq!(name_parts.len(), 1); // TODO handle indexing into table
        if let Some(var) = scope.lock().find_var(&name_parts[0]) {
            Ok(var.val.clone())
        } else {
            let e: RetValOrErr = LuErr::Eval(EvalErr::VarNotFound(SourceCodeItem::new(
                self.syntax().text_range().into(),
                self.syntax().text().to_string(),
            )))
            .into();
            Err(e)
        }
    }
}

impl Evaluable for ArrayExprNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let mut values = Vec::new();
        for val in self.values() {
            values.push(val.evaluate(scope)?);
        }
        Ok(Value::new_array(values))
    }
}

impl Evaluable for TableExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        todo!()
    }
}
