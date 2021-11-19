use enum_as_inner::EnumAsInner;
use lu_error::lu_source_code_item;
use lu_stdx::AMtx;
use lu_syntax::ast::{BareWordToken, NumberExprNode, StringExprNode};
use ordered_float::OrderedFloat;
use parking_lot::RwLock;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};
use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::dbg_state::DbgState;
use crate::{table, Command, CommandCollection, Strct, ValueType};

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
    /// A file name. May contain wildcards
    FileName(String),
    // The following types are lu-copy-on-write (and therefore enclosed in a Rc)
    Array(Rc<Vec<Value>>),
    Optional {
        inner_ty: ValueType,
        val: Option<Box<Value>>,
    },
    // Strcts fields
    // TODO this should contian weak pointer to decl. makes everything easier
    Strct(String, Rc<Vec<(String, Value)>>),
    #[serde(skip)]
    Command(Rc<dyn Command>),
    CommandCollection(CommandCollection),
    /// Not really lu values. But treating them as one, allows us to store them in variables
    #[serde(skip)] // TODO serialize
    StrctDecl(Arc<RwLock<Strct>>),
    #[serde(skip)] // TODO serialize
    DbgState(AMtx<DbgState>),
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
            (Value::Strct(lhs_name, lhs_fields), Value::Strct(rhs_name, rhs_fields)) => {
                lhs_name == rhs_name && lhs_fields == rhs_fields
            }
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
            Value::Strct(name, v) => {
                name.hash(state);
                v.hash(state);
            }
            Value::Command(func) => Rc::as_ptr(func).hash(state),
            Value::CommandCollection(col) => col.hash(state),
            Value::StrctDecl(strct) => Arc::as_ptr(strct).hash(state),
            Value::DbgState(v) => Arc::as_ptr(v).hash(state),
            Value::FileName(v) => v.hash(state),
            Value::Optional { val, .. } => val.hash(state),
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
    pub fn new_strct(name: String, vals: Vec<(String, Value)>) -> Self {
        Value::Strct(name, Rc::new(vals))
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
            Value::FileName(_) => None,
            Value::Number(n) => Some(*n != OrderedFloat::from(0f64)),
            Value::String(s) | Value::BareWord(s) => Some(!s.is_empty()),
            Value::Array(arr) => Some(!arr.is_empty()),
            Value::Command(_) => None,
            Value::StrctDecl(_) => None,
            Value::Strct(_, _) => None,
            Value::CommandCollection(_) => None,
            Value::DbgState(_) => None,
            Value::Optional { .. } => None,
        }
    }

    pub fn is_of_type(&self, ty: &ValueType) -> bool {
        match (self, ty) {
            (Value::Nil, ValueType::Nil)
            | (Value::Bool(_), ValueType::Bool)
            | (Value::Number(_), ValueType::Number)
            | (Value::String(_), ValueType::String)
            | (Value::BareWord(_), ValueType::String)
            | (Value::BareWord(_), ValueType::FileName)
            | (Value::FileName(_), ValueType::String)
            | (Value::FileName(_), ValueType::FileName) => return true,
            (Value::Strct(name, _), ValueType::Strct(strct)) => {
                let strct = Weak::upgrade(strct).unwrap();
                let l_strct = strct.read();
                *name == l_strct.name
            }
            (Value::Command(_), ValueType::Func(_)) => {
                todo!("Assert signatures are the same")
            }
            (Value::Array(inner), ValueType::Array { inner_ty, .. }) => {
                // TODO add array ty to value
                if inner.is_empty() {
                    true
                } else {
                    inner[0].is_of_type(inner_ty)
                }
            }
            _ => false,
        }
    }

    pub fn get_ty(&self) -> ValueType {
        match self {
            Value::Nil => ValueType::Nil,
            Value::Bool(_) => ValueType::Bool,
            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
            Value::BareWord(_) => ValueType::BareWord,
            Value::FileName(_) => ValueType::FileName,
            // TODO better inner_ty
            Value::Array(_) => ValueType::Array {
                inner_ty: Box::new(ValueType::Unspecified),
                inner_ty_decl: lu_source_code_item!(),
            },
            // TODO if strct contains pointer to decl return proper strct here
            Value::Strct(name, _) => ValueType::StrctName(name.clone()),
            Value::Command(cmd) => ValueType::Func(Box::new(cmd.signature().clone())),
            // TODO these should never be reachable
            Value::StrctDecl(_) => todo!("Add pseudo ValueType::StructDecl"),
            Value::DbgState(_) => todo!("Add pseudo ValueType::DbgState"),
            Value::CommandCollection(_) => todo!(),
            Value::Optional { inner_ty, .. } => ValueType::Optional {
                inner_ty: Box::new(inner_ty.clone()),
                inner_ty_decl: lu_source_code_item!(),
            },
        }
    }

    pub fn coerce_to_string(&self) -> Option<&String> {
        match self {
            Value::String(s) | Value::BareWord(s) | Value::FileName(s) => Some(s),
            _ => None,
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
            Value::Array(arr) => {
                if arr.is_empty() {
                    write!(f, "")
                } else if arr[0].as_strct().is_some() {
                    write!(f, "{}", table::to_fmt_table(arr))
                } else {
                    write!(f, "{:?}", arr)
                }
            }
            Value::Command(v) => write!(f, "Command: {} {:?}", v.name(), v.signature_item()),
            Value::StrctDecl(v) => write!(f, "{:p}", Arc::as_ptr(v)),
            Value::Strct(name, fields) => write!(f, "{}{:?}", name, fields),
            // TODO nice display here
            Value::CommandCollection(col) => write!(f, "{:?}", col),
            Value::DbgState(dbg_state) => write!(f, "{:?}", dbg_state),
            Value::FileName(s) => write!(f, "{}", s),
            Value::Optional { val, .. } => match val {
                Some(val) => write!(f, "Some({})", val),
                None => write!(f, "None"),
            },
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

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Number((v as f64).into())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
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
