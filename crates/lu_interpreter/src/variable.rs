use derive_more::From;
use lu_error::SourceCodeItem;
use lu_syntax::{
    ast::{ArgSignatureNode, CmdStmtNode, FnStmtNode, ForStmtNode, LetStmtNode, StructStmtNode},
    AstNode, AstToken,
};
use lu_syntax_elements::constants::IN_ARG_NAME;
use lu_value::Value;
use serde::{Deserialize, Serialize};

use crate::{Callable, Command, Function, Strct};

#[derive(Educe)]
#[educe(Default)]
#[derive(Clone, Debug, Eq, PartialEq, new, Hash, From)]
pub enum VarDeclNode {
    #[educe(Default)]
    Dummy,
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    StrctStmt(StructStmtNode),
    /// For stmt with usize being index into exact param
    ForStmt(ForStmtNode, usize),
    ArgSignature(ArgSignatureNode),
    // For $in (before it is mapped to the correct name)
    PrevCmdStmt(CmdStmtNode),
    ErrorUsage(SourceCodeItem),
}

impl VarDeclNode {
    pub fn to_item(&self) -> SourceCodeItem {
        match self {
            VarDeclNode::LetStmt(n) => n.to_item(),
            VarDeclNode::FnStmt(n) => n.decl_item(),
            VarDeclNode::ArgSignature(n) => n.to_item(),
            VarDeclNode::ForStmt(n, i) => n.var_names()[i.clone()].to_item(),
            VarDeclNode::Dummy => SourceCodeItem::tmp_todo_item(),
            VarDeclNode::PrevCmdStmt(n) => n.to_item(),
            VarDeclNode::StrctStmt(n) => n.to_item(),
            VarDeclNode::ErrorUsage(item) => item.clone(),
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
    pub decl: VarDeclNode, // TODO doesn't need to be optional, decl can be in rust code
}

impl Variable {
    pub fn new_func(func: Function, decl: FnStmtNode) -> Variable {
        let func: Callable = func.into();
        Variable::new(
            func.name().to_string(),
            Value::new_func(func),
            VarDeclNode::FnStmt(decl),
        )
    }

    pub fn new_struct(strct: Strct, decl: StructStmtNode) -> Variable {
        Variable::new(
            strct.name.clone(),
            Value::new_strct(strct),
            VarDeclNode::StrctStmt(decl),
        )
    }

    pub fn new_in(val: Value, decl: VarDeclNode) -> Self {
        Self {
            name: IN_ARG_NAME.to_string(),
            val,
            decl,
        }
    }
    pub fn new_args(val: Value) -> Self {
        Self {
            name: "args".into(),
            val,
            // TODO correct val
            decl: VarDeclNode::Dummy,
        }
    }

    pub fn val_as_callable(&self) -> Option<&Callable> {
        self.val.as_function().map(|func| {
            func.downcast_ref::<Callable>()
                .expect("Func is always castable to Callable")
        })
    }
    pub fn val_as_strct(&self) -> Option<&Strct> {
        self.val.as_strct().map(|strct| {
            strct
                .downcast_ref::<Strct>()
                .expect("Struct-Var is always castable to Struct")
        })
    }
}
