#![allow(dead_code)]
use crate::scope::ScopeFrameId;
use crate::{Command, Evaluable, VarDeclNode, Variable};
use crate::{Evaluator, ValueType};
use lu_syntax::ast::{
    FlagSignatureNode, FnStmtNode, InSignatureNode, ParamSignatureNode, RetSignatureNode,
    VarArgParamSignatureRuleNode,
};
use lu_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, new)]
pub struct ArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
    pub is_opt: bool,
    pub decl: Option<ParamSignatureNode>,
}

impl Into<Variable> for ArgSignature {
    fn into(self) -> Variable {
        Variable::new(
            self.name,
            Value::Nil,
            self.decl.map(|n| VarDeclNode::ArgSignature(n)),
        )
    }
}

#[derive(Clone, Debug, new)]
pub struct VarArgSignature {
    pub name: String,
    pub type_: Option<ValueType>,
    pub decl: Option<VarArgParamSignatureRuleNode>,
}

impl Into<Variable> for VarArgSignature {
    fn into(self) -> Variable {
        Variable::new(
            self.name,
            Value::Nil,
            self.decl.map(|n| VarDeclNode::VarArgSignature(n)),
        )
    }
}

#[derive(Clone, Debug, new)]
pub struct InArgSignature {
    pub type_: Option<ValueType>,
    pub decl: Option<InSignatureNode>,
}

impl Into<Variable> for InArgSignature {
    fn into(self) -> Variable {
        Variable::new(
            "in".into(),
            Value::Nil,
            self.decl.map(|n| VarDeclNode::InArgSignature(n)),
        )
    }
}

#[derive(Clone, Debug, new)]
pub struct RetArgSignature {
    pub type_: Option<ValueType>,
    pub decl: Option<RetSignatureNode>,
}

impl Into<Variable> for RetArgSignature {
    fn into(self) -> Variable {
        Variable::new(
            "ret".into(),
            Value::Nil,
            self.decl.map(|n| VarDeclNode::RetArgSignature(n)),
        )
    }
}

#[derive(Clone, Debug, new, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlagSignature {
    pub long_name: Option<String>,
    pub short_name: Option<char>,
    pub type_: Option<ValueType>,
    #[new(default)] // TODO this default should be false, making every flag necessary
    pub is_opt: bool,
    #[serde(skip)]
    pub decl: Option<FlagSignatureNode>,
}

#[derive(Clone, Debug, new)]
pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub var_arg: Option<VarArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub in_type: Option<InArgSignature>,
    pub ret_type: Option<RetArgSignature>,
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
        state: &mut Evaluator,
    ) -> lu_error::LuResult<lu_value::Value> {
        // TODO typecheck and put vars into scope
        if let Some(block) = self.fn_node.block_stmt() {
            block.evaluate(state)
        } else {
            Ok(Value::Nil)
        }
    }
}
