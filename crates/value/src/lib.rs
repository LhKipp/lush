use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    BareWord(String),
    Array(Vec<Value>),
}
