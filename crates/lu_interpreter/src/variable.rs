use lu_syntax::ast::{FnStmtNode, ForStmtNode, LetStmtNode, ParamSignatureNode};
use lu_value::Value;
use serde::{Deserialize, Serialize};

use crate::{Callable, Command, Function};

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub enum VarDeclNode {
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    /// For stmt with usize being index into exact param
    ForStmt(ForStmtNode, usize),
    ArgSignature(ParamSignatureNode),
}

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub val: Value,
    #[serde(skip)]
    pub decl: Option<VarDeclNode>,
}

impl Variable {
    pub fn new_func(func: Function, decl: FnStmtNode) -> Variable {
        let func: Callable = func.into();
        Variable::new(
            func.name().to_string(),
            Value::new_func(func),
            Some(VarDeclNode::FnStmt(decl)),
        )
    }

    pub fn new_in(val: Value) -> Self {
        Self {
            name: "in".into(),
            val,
            decl: None,
        }
    }
    pub fn new_args(val: Value) -> Self {
        Self {
            name: "args".into(),
            val,
            decl: None,
        }
    }
}
