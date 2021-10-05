#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::Value;
use lu_syntax::{
    ast::{
        BlockStmtNode, ConditionElement, IfBlockNode, IfStmtNode, MathExprNode, OperatorExprElement,
    },
    AstElement, AstToken,
};

use crate::{EvalArg, Evaluable, Evaluator, Interpreter, ScopeFrameTag};

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

// #[cfg(test)]
// mod test {
//     use lu_error::LuResult;
//     use lu_syntax::ast::SourceFileNode;
//     use lu_test_support::{init_logger, make_test_interpreter};
//     use lu_text_util::SourceCode;
//     use lu_interpreter_structs::Value;
//     use {conformance, serde_json};

//     #[conformance::tests(exact, serde=serde_json, file="test_data/evaluate/if_stmt/single_if.json_test")]
//     fn general_interpreter_tests(s: &str) -> LuResult<Value> {
//         init_logger();
//         let mut evaluator = make_test_interpreter();

//         evaluator.eval(SourceCode::Text(s.to_string()))
//     }
// }
