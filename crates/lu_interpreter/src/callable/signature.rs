use lu_value::ValueType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgModifier {
    Optional,
}

#[derive(Clone, Debug)]
pub struct ArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
    pub modifiers: Vec<ArgModifier>,
}

impl ArgSignature {
    #[allow(dead_code)]
    fn is_opt(&self) -> bool {
        self.modifiers.contains(&ArgModifier::Optional)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FlagModifier {
    Required,
}

#[derive(Clone, Debug)]
pub struct FlagSignature {
    pub short_name: String,
    pub long_name: String,
    pub modifiers: Vec<FlagModifier>,
}

#[derive(Clone, Debug)]
pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub ret_type: ValueType,
    pub input_type: ValueType,
}

impl Signature {}

impl Default for Signature {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            flags: Vec::new(),
            ret_type: ValueType::Any,
            input_type: ValueType::Any,
        }
    }
}
