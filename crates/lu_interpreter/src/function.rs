#![allow(dead_code)]
use crate::command::RunExternalCmd;
use crate::{scope::ScopeFrameId, Command};
use crate::{EvalArg, Evaluable, Variable};
use lu_parser::grammar::FnStmtRule;
use lu_syntax::{ast::FnStmtNode, Parse};
use lu_value::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArgModifier {
    Optional,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum FlagModifier {
    Required,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagSignature {
    pub short_name: String,
    pub long_name: String,
    pub modifiers: Vec<FlagModifier>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

fn default_fn_stmt_node() -> FnStmtNode {
    let parse_result = Parse::rule(
        "
        fn default_fn [] 
        end
        ",
        &FnStmtRule {},
    );
    parse_result.ok::<FnStmtNode>().unwrap()
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

#[derive(Clone, Debug)]
pub enum Callable {
    Func(Function),
    InternalCmd(Box<dyn Command>),
    ExternalCmd(RunExternalCmd),
}

impl Callable {}

impl Command for Callable {
    fn do_run(
        &self,
        _: &[EvalArg],
        state: &mut crate::Interpreter,
    ) -> lu_error::LuResult<lu_value::Value> {
        match self {
            Callable::Func(f) => f.evaluate(state),
            Callable::InternalCmd(cmd) => cmd.run(state),
            Callable::ExternalCmd(cmd) => cmd.run(state),
        }
    }

    fn name(&self) -> &str {
        match self {
            Callable::Func(f) => &f.name,
            Callable::InternalCmd(cmd) => cmd.name(),
            Callable::ExternalCmd(cmd) => cmd.name(),
        }
    }
}

impl From<Box<dyn Command>> for Callable {
    fn from(cmd: Box<dyn Command>) -> Self {
        Callable::InternalCmd(cmd)
    }
}

impl From<Function> for Callable {
    fn from(func: Function) -> Self {
        Callable::Func(func)
    }
}
