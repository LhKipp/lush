use lu_syntax::ast::{ForStmtNode, LetStmtNode, ParamSignatureNode};
use lu_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub enum VarDeclNode {
    LetStmt(LetStmtNode),
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
