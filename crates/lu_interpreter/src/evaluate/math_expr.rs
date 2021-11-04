#![allow(unused_imports)]
use crate::evaluate::eval_prelude::*;
use contracts::ensures;
use lu_syntax::ast::{
    ConditionElement, IfBlockNode, IfStmtNode, MathExprNode, OperatorExprElement, StrctStmtNode,
    ValueExprElement,
};

impl Evaluable for MathExprNode {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let lhs = self.lhs();
        let rhs = self.rhs();
        let lhs_val = lhs.evaluate(scope)?;
        let rhs_val = rhs.evaluate(scope)?;

        match self.operator() {
            OperatorExprElement::AssignSign(_) => {
                let lhs_var = if let ValueExprElement::ValuePathExpr(e) = lhs {
                    e
                } else {
                    todo!("Error out");
                };
                assert!(lhs_var.var_name_parts_with_item().len() == 1);
                let (var_name, usage) = lhs_var.var_name_parts_with_item()[0].clone();
                let mut l_scope = scope.lock();
                let var =
                    Evaluator::lu_result_to_eval_result(l_scope.expect_var_mut(&var_name, usage))?;

                var.val = rhs_val;

                // Assignment does not return value
                Ok(Value::Nil)
            }
            OperatorExprElement::PlusSign(_) => return eval_plus_sign(lhs_val, rhs_val),
            OperatorExprElement::MinusSign(_) => return eval_minus_sign(lhs_val, rhs_val),
            OperatorExprElement::MultSign(_) => return eval_mult_sign(lhs_val, rhs_val),
            OperatorExprElement::DivSign(_) => return eval_div_sign(lhs_val, rhs_val),

            OperatorExprElement::LessThanSign(_) => Ok((lhs_val < rhs_val).into()),
            OperatorExprElement::LessOrEqualSign(_) => Ok((lhs_val <= rhs_val).into()),
            OperatorExprElement::EqualitySign(_) => Ok((lhs_val == rhs_val).into()),
            OperatorExprElement::InequalitySign(_) => Ok((lhs_val != rhs_val).into()),
            OperatorExprElement::BiggerThanSign(_) => Ok((lhs_val > rhs_val).into()),
            OperatorExprElement::BiggerOrEqualSign(_) => Ok((lhs_val >= rhs_val).into()),

            OperatorExprElement::DivAssignSign(_) => todo!(),
            OperatorExprElement::MulAssignSign(_) => todo!(),
            OperatorExprElement::AddAssignSign(_) => todo!(),
            OperatorExprElement::MinAssignSign(_) => todo!(),
        }
    }
}

fn eval_plus_sign(lhs: Value, rhs: Value) -> EvalResult {
    match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
        _ => todo!(),
    }
}
fn eval_minus_sign(_lhs: Value, _rhs: Value) -> EvalResult {
    todo!()
}
fn eval_mult_sign(_lhs: Value, _rhs: Value) -> EvalResult {
    todo!()
}
fn eval_div_sign(_lhs: Value, _rhs: Value) -> EvalResult {
    todo!()
}
