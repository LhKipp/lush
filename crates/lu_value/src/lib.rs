use ordered_float::OrderedFloat;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    // Lu has value semantics. All the time! This allows for easier reasoning about
    // pure functions with inputs. However, copying large structs (Array, Table, ...)
    // may create a lot of copying. Therefore we use the COW-Idiom, by leveraging Rc::make_mut
    //
    // The following types are lu-copy
    Nil,
    Number(OrderedFloat<f64>),
    String(String),
    BareWord(String),

    // The following types are lu-copy-on-write (and therefore enclosed in a Rc)
    Array(Rc<Vec<Value>>),
}

impl Value {
    pub fn new_array(vals: Vec<Value>) -> Self {
        Value::Array(Rc::new(vals))
    }
}
