use enum_as_inner::EnumAsInner;
use lu_syntax::ast::{BareWordToken, NumberExprNode, StringExprNode};
use ordered_float::OrderedFloat;
use parking_lot::RwLock;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{Command, Strct};

// pub const NIL_VAL: Value = Value::Nil;

// TODO move this to lu_interpreter_structs
#[derive(Clone, Serialize, Deserialize, EnumAsInner)]
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
    Command(Rc<dyn Command>),

    /// Not really lu values. But treating them as one, allows us to store them in variables
    #[serde(skip)] // TODO serialize
    StrctDecl(Arc<RwLock<Strct>>),
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
            (Value::Command(lhs), Value::Command(rhs)) => Rc::ptr_eq(lhs, rhs),
            _ => false,
        }
    }
}
impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Nil, Value::Nil) => Some(Ordering::Equal),
            (Value::Bool(l), Value::Bool(r)) => l.partial_cmp(r),
            (Value::Number(l), Value::Number(r)) => l.partial_cmp(r),
            (Value::String(l), Value::String(r)) => l.partial_cmp(r),
            (Value::BareWord(l), Value::BareWord(r)) => l.partial_cmp(r),
            (Value::Array(_), Value::Array(_)) => None,
            (Value::Command(_), Value::Command(_)) => None,
            (Value::StrctDecl(_), Value::StrctDecl(_)) => None,
            _ => {
                unreachable!("Caught by ty checker");
            }
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => {
                state.write_i32(1);
                state.finish();
            }
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::BareWord(v) => v.hash(state),
            Value::Array(v) => v.hash(state),
            Value::Command(func) => Rc::as_ptr(func).hash(state),
            Value::StrctDecl(strct) => Arc::as_ptr(strct).hash(state),
        }
    }
}

impl Value {
    pub fn new_func(func: Rc<dyn Command>) -> Self {
        Value::Command(func)
    }
    pub fn new_strct_decl(strct: Strct) -> Self {
        Value::StrctDecl(Arc::new(RwLock::new(strct)))
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

    pub fn is_func(&self) -> bool {
        matches!(self, Value::Command(_))
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
            Value::Command(_) => None,
            Value::StrctDecl(_) => None,
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Command(v) => write!(f, "Command: {} {:?}", v.name(), v.signature_item()),
            Value::StrctDecl(v) => write!(f, "{:p}", Arc::as_ptr(v)),
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

impl From<&BareWordToken> for Value {
    fn from(n: &BareWordToken) -> Self {
        Value::BareWord(n.value())
    }
}

impl From<&StringExprNode> for Value {
    fn from(n: &StringExprNode) -> Self {
        Value::String(n.value())
    }
}

impl From<&NumberExprNode> for Value {
    fn from(n: &NumberExprNode) -> Self {
        Value::Number(n.value().into())
    }
}
