use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueType {
    Unspecified,
    Any,
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array(Box<ValueType>),
    Function,
    /// Type with name string. Could not yet been deduced
    Unresolved(String),
}
