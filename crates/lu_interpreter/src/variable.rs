use lu_error::SourceCodeItem;
use lu_syntax::{
    ast::{ArgSignatureNode, FnStmtNode, ForStmtNode, LetStmtNode},
    AstNode, AstToken,
};
use lu_value::Value;
use serde::{Deserialize, Serialize};

use crate::{Callable, Command, Function};

#[derive(Clone, Debug, Eq, PartialEq, new, Hash)]
pub enum VarDeclNode {
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    /// For stmt with usize being index into exact param
    ForStmt(ForStmtNode, usize),
    ArgSignature(ArgSignatureNode),
}

impl VarDeclNode {
    pub fn into_item(&self) -> SourceCodeItem {
        match self {
            VarDeclNode::LetStmt(n) => n.into_item(),
            VarDeclNode::FnStmt(n) => n.into_item(),
            VarDeclNode::ArgSignature(n) => n.into_item(),
            VarDeclNode::ForStmt(n, i) => n.var_names()[i.clone()].into_item(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize, Hash)]
pub struct Variable {
    /// The name of the variable
    pub name: String,
    /// The evaluation value of this variable, Value::Nil in other stages of interpretation
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

    pub fn val_as_callable(&self) -> Option<&Callable> {
        self.val.as_function().map(|func| {
            func.downcast_ref::<Callable>()
                .expect("Func is always castable to Callable")
        })
    }
}
