use std::rc::Rc;

use derive_more::From;
use lu_error::SourceCodeItem;
use lu_syntax::{
    ast::{CmdStmtNode, FnStmtNode, ForStmtNode, LetStmtNode, StrctStmtNode},
    AstNode, AstToken,
};
use lu_syntax_elements::constants::IN_ARG_NAME;
use serde::{Deserialize, Serialize};

use crate::{Command, CommandCollection, Strct, Value};

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
    pub fn new_func(func: Rc<dyn Command>) -> Variable {
        // TODO better decl here
        let decl = func.signature().decl.clone();
        Variable::new(
            func.name().to_string(),
            Value::new_func(func),
            VarDeclNode::CatchAll(decl),
        )
    }

    pub fn new_func_collection(funcs: Vec<Rc<dyn Command>>) -> Variable {
        // TODO better decl here
        let collection = CommandCollection::new(funcs);
        let name = collection.name();
        let decl = collection.pseudo_decl();
        Variable::new(
            name.to_string(),
            Value::CommandCollection(collection),
            VarDeclNode::CatchAll(decl),
        )
    }

    pub fn new_strct_decl(strct: Strct) -> Variable {
        let decl = strct.decl.clone();
        Variable::new(
            strct.name.clone(),
            Value::new_strct_decl(strct),
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
}
