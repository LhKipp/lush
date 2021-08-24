#![allow(dead_code)]
use crate::scope::ScopeFrameId;
use crate::Variable;
use lu_syntax::ast::FnStmtNode;
use lu_value::ValueType;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArgModifier {
    Optional,
}

pub struct ArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
    pub modifiers: Vec<ArgModifier>,
}

impl ArgSignature {
    fn is_opt(&self) -> bool {
        self.modifiers.contains(&ArgModifier::Optional)
    }
}

pub enum FlagModifier {
    Required,
}

pub struct FlagSignature {
    pub short_name: String,
    pub long_name: String,
    pub modifiers: Vec<FlagModifier>,
}

pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub ret_type: ValueType,
    pub input_type: ValueType,
}

pub struct Function {
    pub name: String,
    pub signature: Signature,
    pub scope_frame_id: ScopeFrameId,
    pub fn_node: FnStmtNode,
    // For closures only
    pub captured_vars: Vec<Variable>,
}
