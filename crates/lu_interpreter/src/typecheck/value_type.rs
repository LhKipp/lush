use serde::{Deserialize, Serialize};

// enum ParamType{
//     GenericT(i32),
//     Concrete(ValueType),
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// struct FuncType{
//     name: String,
//     req_flags: Vec<String>,

//     args: Vec<ParamType>,
//     ret_t: ParamType
// }

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
    // Function(FuncType),
}
