use ordered_float::OrderedFloat;
use std::{any::Any, fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    Any,
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array,
    Function,
}

pub const NIL_VAL: Value = Value::Nil;

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
    #[serde(skip)]
    Function(Rc<dyn Any>),
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
    pub fn new_func<F: Any + Sized>(func: F) -> Self {
        Value::Function(Rc::new(func))
    }

    pub fn new_array(vals: Vec<Value>) -> Self {
        Value::Array(Rc::new(vals))
    }

    pub fn expect_array(&mut self) -> &mut Vec<Value> {
        match self {
            Value::Array(vals) => Rc::make_mut(vals),
            _ => unreachable!(),
        }
    }

    pub fn is_nil(&self) -> bool {
        return self == &Value::Nil;
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
            Value::Function(_) => None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(v) => v.fmt(f),
            Value::Number(v) => v.fmt(f),
            Value::String(v) => v.fmt(f),
            Value::BareWord(v) => v.fmt(f),
            Value::Function(v) => write!(f, "{:p}", Rc::as_ptr(v)),
            Value::Array(arr) => write!(f, "{:?}", arr),
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
