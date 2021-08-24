use lu_value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub val: Value,
    pub name: String,
}
