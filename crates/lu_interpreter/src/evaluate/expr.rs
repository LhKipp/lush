use lu_error::{EvalErr, LuErr, SourceCodeItem};
use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, CmdOrValueExprElement, NumberExprNode, StringExprNode,
        TableExprNode, ValueExprElement, ValuePathExprNode,
    },
    AstNode, AstToken,
};
use lu_value::Value;

use crate::{EvalArg, EvalResult, Evaluable, Evaluator, RetValOrErr};

impl Evaluable for ValueExprElement {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        match self {
            ValueExprElement::BareWord(n) => n.evaluate(state),
            ValueExprElement::NumberExpr(n) => n.evaluate(state),
            ValueExprElement::MathExpr(n) => n.evaluate(state),
            ValueExprElement::StringExpr(n) => n.evaluate(state),
            ValueExprElement::ValuePathExpr(n) => n.evaluate(state),
            ValueExprElement::ArrayExpr(n) => n.evaluate(state),
            ValueExprElement::TableExpr(n) => n.evaluate(state),
            ValueExprElement::StrctCtorExpr(_) => todo!(),
        }
    }
}

impl Evaluable for CmdOrValueExprElement {
    fn do_evaluate(&self, args: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        match self {
            CmdOrValueExprElement::CmdStmt(n) => n.evaluate_with_args(args, state),
            CmdOrValueExprElement::PipedCmdsStmt(n) => n.evaluate_with_args(args, state),
            CmdOrValueExprElement::ValueExpr(n) => n.evaluate_with_args(args, state),
        }
    }
}

impl Evaluable for BareWordToken {
    fn do_evaluate(&self, _: &[EvalArg], _: &mut Evaluator) -> EvalResult {
        Ok(Value::BareWord(self.text().to_string()))
    }
}

impl Evaluable for NumberExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _state: &mut Evaluator) -> EvalResult {
        Ok(self.value())
    }
}

impl Evaluable for StringExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _state: &mut Evaluator) -> EvalResult {
        Ok(Value::String(self.text().to_string()))
    }
}

impl Evaluable for ValuePathExprNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        let name_parts = self.var_name_parts();
        assert_eq!(name_parts.len(), 1); // TODO handle indexing into table
        if let Some(var) = state.scope.lock().find_var(&name_parts[0]) {
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
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        let mut values = Vec::new();
        for val in self.values() {
            values.push(val.evaluate(state)?);
        }
        Ok(Value::new_array(values))
    }
}

impl Evaluable for TableExprNode {
    fn do_evaluate(&self, _: &[EvalArg], _state: &mut Evaluator) -> EvalResult {
        todo!()
    }
}
