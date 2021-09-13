#![allow(dead_code)]
use crate::scope::ScopeFrameId;
use crate::typecheck::ValueType;
use crate::{Command, Evaluable, Variable};
use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

#[derive(Clone, Debug, new)]
pub struct ArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
    pub is_opt: bool,
}

impl ArgSignature {
    pub fn is_in_arg(&self) -> bool {
        self.name == "in"
    }
    pub fn is_ret_arg(&self) -> bool {
        self.name == "ret"
    }
}

#[derive(Clone, Debug, new)]
pub struct VarArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
}

#[derive(Clone, Debug, new, Hash, Eq, PartialEq)]
pub struct FlagSignature {
    pub long_name: Option<String>,
    pub short_name: Option<char>,
    pub type_: Option<ValueType>,
    #[new(default)]
    pub is_opt: bool,
}

#[derive(Clone, Debug, new)]
pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub var_arg: Option<VarArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub in_type: Option<ValueType>,
    pub ret_type: Option<ValueType>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub signature: Signature,
    pub fn_node: FnStmtNode,
    pub parent_frame_id: ScopeFrameId,
    // For closures only
    pub captured_vars: Vec<Variable>,
}

impl Function {
    pub fn new(
        name: String,
        signature: Signature,
        fn_node: FnStmtNode,
        parent_frame_id: ScopeFrameId,
    ) -> Self {
        Self {
            name,
            signature,
            parent_frame_id,
            fn_node,
            captured_vars: Vec::new(),
        }
    }
}

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run(
        &self,
        _: &[crate::EvalArg],
        state: &mut crate::Interpreter,
    ) -> lu_error::LuResult<lu_value::Value> {
        // TODO typecheck and put vars into scope
        if let Some(block) = self.fn_node.block_stmt() {
            block.evaluate(state)
        } else {
            Ok(Value::Nil)
        }
    }
}
