#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{
        BlockStmtNode, ConditionElement, IfBlockNode, IfStmtNode, MathExprNode, OperatorExprElement,
    },
    AstElement, AstToken,
};
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, Interpreter, ScopeFrameTag};

impl Evaluable for MathExprNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let lhs = self.lhs().unwrap();
        let rhs = self.rhs().unwrap();
        let op = self.operator().unwrap();

        let lhs = lhs.evaluate(state)?;
        let rhs = rhs.evaluate(state)?;

        match op {
            OperatorExprElement::PlusSign(_) => return eval_plus_sign(lhs, rhs),
            OperatorExprElement::MinusSign(_) => return eval_minus_sign(lhs, rhs),
            OperatorExprElement::MultSign(_) => return eval_mult_sign(lhs, rhs),
            OperatorExprElement::DivSign(_) => return eval_div_sign(lhs, rhs),
            OperatorExprElement::LessThanSign(_) => return eval_less_than_sign(lhs, rhs),
            OperatorExprElement::LessOrEqualSign(_) => return eval_less_or_equal_sign(lhs, rhs),
            OperatorExprElement::EqualitySign(_) => return eval_equality_sign(lhs, rhs),
            OperatorExprElement::InequalitySign(_) => return eval_inequality_sign(lhs, rhs),
            OperatorExprElement::BiggerThanSign(_) => return eval_bigger_than_sign(lhs, rhs),
            OperatorExprElement::BiggerOrEqualSign(_) => {
                return eval_bigger_or_equal_sign(lhs, rhs)
            }
            OperatorExprElement::AssignSign(_) => return eval_assign_sign(lhs, rhs),
            OperatorExprElement::RightStream(_) => return eval_right_stream(lhs, rhs),
        }
    }
}

fn eval_plus_sign(lhs: Value, rhs: Value) -> LuResult<Value> {
    match (lhs, rhs) {
        (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
        _ => todo!(),
    }
}
fn eval_minus_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_mult_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_div_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_less_than_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_less_or_equal_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_equality_sign(lhs: Value, rhs: Value) -> LuResult<Value> {
    Ok(Value::Bool(lhs == rhs))
}
fn eval_inequality_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_bigger_than_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_bigger_or_equal_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_assign_sign(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
fn eval_right_stream(_lhs: Value, _rhs: Value) -> LuResult<Value> {
    todo!()
}
