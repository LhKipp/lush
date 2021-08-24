use ordered_float::OrderedFloat;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array,
    Function,
}

// Empty Trait to mark something as a function
#[typetag::serde(tag = "type")]
pub trait Func: std::fmt::Debug {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    // Lu has value semantics. All the time! This allows for easier reasoning about
    // pure functions with inputs. However, copying large structs (Array, Table, ...)
    // may create a lot of copying. Therefore we use the COW-Idiom, by leveraging Rc::make_mut
    //
    // The following types are lu-copy
    Nil,
    Bool(bool),
    Number(OrderedFloat<f64>),
    String(String),
    BareWord(String),

    // The following types are lu-copy-on-write (and therefore enclosed in a Rc)
    Array(Rc<Vec<Value>>),
    Function(Rc<dyn Func>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (other, self) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(lhs), Value::Bool(rhs)) => lhs == rhs,
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::BareWord(lhs), Value::BareWord(rhs)) => lhs == rhs,
            (Value::Array(lhs), Value::Array(rhs)) => lhs == rhs,
            (Value::Function(lhs), Value::Function(rhs)) => Rc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}
impl Eq for Value {}

impl Value {
    pub fn new_array(vals: Vec<Value>) -> Self {
        Value::Array(Rc::new(vals))
    }

    pub fn expect_array(&mut self) -> &mut Vec<Value> {
        match self {
            Value::Array(vals) => Rc::make_mut(vals),
            _ => unreachable!(),
        }
    }

    /// Returns Some(true|false) if self represents a true or false value
    /// Returns None if self is not convertible to bool
    pub fn convert_to_bool(&self) -> Option<bool> {
        // TODO check what else should be false / true
        match self {
            Value::Nil => Some(false),
            Value::Bool(v) => Some(*v),
            Value::Number(n) => Some(*n != OrderedFloat::from(0f64)),
            Value::String(s) | Value::BareWord(s) => Some(!s.is_empty()),
            Value::Array(arr) => Some(!arr.is_empty()),
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<OrderedFloat<f64>> for Value {
    fn from(v: OrderedFloat<f64>) -> Self {
        Value::Number(v)
    }
}
