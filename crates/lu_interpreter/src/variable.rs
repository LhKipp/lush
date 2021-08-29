use lu_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub val: Value,
}

impl Variable {
    pub fn new_in(val: Value) -> Self {
        Self {
            name: "in".into(),
            val,
        }
    }
}
