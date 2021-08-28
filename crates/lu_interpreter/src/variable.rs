use lu_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub val: Value,
}
