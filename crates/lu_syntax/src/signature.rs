use lu_value::ValueType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ArgModifier {
    Optional,
}

#[derive(Clone, Debug, new)]
pub struct ArgSignature {
    pub name: String,
    pub type_: ValueType,
    pub is_opt: bool,
}

#[derive(Clone, Debug, new)]
pub struct VarArgSignature {
    pub name: String,
    pub type_: ValueType,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum FlagModifier {
    Required,
}

#[derive(Clone, Debug, new)]
pub struct FlagSignature {
    pub long_name: String,
    pub short_name: String,
    pub is_opt: bool,
}

#[derive(Clone, Debug)]
pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub var_arg: Option<VarArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub ret_type: ValueType,
    pub in_type: ValueType,
}

impl Signature {}

impl Default for Signature {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            flags: Vec::new(),
            // TODO check whether any is correct and not unspecified
            ret_type: ValueType::Any,
            in_type: ValueType::Any,
            var_arg: None,
        }
    }
}
