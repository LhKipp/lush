use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::{MathExprNode, OperatorExprElement, ValueExprElement};

impl Evaluable for MathExprNode {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        // As cast is handled special
        let operator = self.operator();
        if let OperatorExprElement::AsKeyword(_) = operator {
            let lhs = self.lhs();
            let lhs_val = lhs.evaluate(scope)?;
            let rhs_ty = ValueType::from_node_or_err_resolve_strct_name(
                &self.rhs_as_lu_type().unwrap().into_type(),
                &scope.lock(),
            )
            .as_results();
            let rhs_ty = Evaluator::lu_results_to_eval_result(rhs_ty)?;
            if !lhs_val.is_of_type(&rhs_ty) {
                let err: LuErr = EvalErr::BadCast {
                    cast_math_expr: self.to_item(),
                    value_item: lhs.to_item(),
                    value_ty: lhs_val.get_ty().to_string(),
                    expected_ty: rhs_ty.to_string(),
                }
                .into();
                return Err(err.into());
            } else {
                return Ok(lhs_val);
            }
        }

        let lhs = self.lhs();
        let rhs = self.rhs();
        let lhs_val = lhs.evaluate(scope)?;
        let rhs_val = rhs.evaluate(scope)?;

        match operator {
            OperatorExprElement::AsKeyword(_) => {
                unreachable!("Handled above")
            }
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
            OperatorExprElement::PlusSign(_) => {
                let l_val = lhs_val.as_number().unwrap();
                let r_val = rhs_val.as_number().unwrap();
                Ok(Value::Number(l_val + r_val))
            }
            OperatorExprElement::MinusSign(_) => {
                let l_val = lhs_val.as_number().unwrap();
                let r_val = rhs_val.as_number().unwrap();
                Ok(Value::Number(l_val - r_val))
            }
            OperatorExprElement::MultSign(_) => {
                let l_val = lhs_val.as_number().unwrap();
                let r_val = rhs_val.as_number().unwrap();
                Ok(Value::Number(l_val * r_val))
            }
            OperatorExprElement::DivSign(_) => {
                let l_val = lhs_val.as_number().unwrap();
                let r_val = rhs_val.as_number().unwrap();
                Ok(Value::Number(l_val / r_val))
            }
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
