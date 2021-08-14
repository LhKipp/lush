use value::Value;

use crate::{evaluation_error::EvalErr, Interpreter};

mod cmd_call;

pub trait Evaluable {
    fn evaluate(&self, state: &mut Interpreter) -> (Value, EvalErr);
}
