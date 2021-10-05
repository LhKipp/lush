use derive_more::From;
use lu_error::SourceCodeItem;
use lu_syntax::{
    ast::{CmdStmtNode, FnStmtNode, ForStmtNode, LetStmtNode, StrctStmtNode},
    AstNode, AstToken,
};
use lu_syntax_elements::constants::IN_ARG_NAME;
use serde::{Deserialize, Serialize};

use crate::{Command, Strct, UsePath, Value};

#[derive(Educe)]
#[educe(Default)]
#[derive(Clone, Debug, Eq, PartialEq, new, Hash, From)]
pub enum VarDeclNode {
    #[educe(Default)]
    Dummy,
    LetStmt(LetStmtNode),
    FnStmt(FnStmtNode),
    StrctStmt(StrctStmtNode),
    /// For stmt with usize being index into exact param
    ForStmt(ForStmtNode, usize),
    // For $in (before it is mapped to the correct name)
    PrevCmdStmt(CmdStmtNode),
    // Used for errors and arg signature and others :)
    CatchAll(SourceCodeItem),
}

impl VarDeclNode {
    pub fn to_item(&self) -> SourceCodeItem {
        match self {
            VarDeclNode::LetStmt(n) => n.item_till_assign(),
            VarDeclNode::FnStmt(n) => n.decl_item(),
            VarDeclNode::ForStmt(n, i) => n.var_names()[i.clone()].to_item(),
            VarDeclNode::Dummy => SourceCodeItem::tmp_todo_item(),
            VarDeclNode::PrevCmdStmt(n) => n.to_item(),
            VarDeclNode::StrctStmt(n) => n.to_item(),
            VarDeclNode::CatchAll(item) => item.clone(),
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
    pub decl: VarDeclNode,
}

impl Variable {
    pub fn new_func(func: Box<dyn Command>) -> Variable {
        // TODO better decl here
        let decl = func.signature().decl.clone();
        Variable::new(
            func.name().to_string(),
            Value::new_func(func),
            VarDeclNode::CatchAll(decl),
        )
    }

    pub fn new_strct(strct: Strct) -> Variable {
        let decl = strct.decl.clone();
        Variable::new(
            strct.name.clone(),
            Value::new_strct(strct),
            VarDeclNode::CatchAll(decl),
        )
    }

    pub fn new_use_path(use_path: UsePath) -> Variable {
        let decl = use_path.decl.clone();
        // TODO better decl here
        Variable::new(
            use_path.to_string(),
            Value::new_use_path(use_path),
            VarDeclNode::CatchAll(decl),
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

    pub fn val_as_callable(&self) -> Option<&Box<dyn Command>> {
        self.val.as_function().map(|func| {
            func.downcast_ref::<Box<dyn Command>>()
                .expect("Func is always castable to Box<dyn Command>")
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
