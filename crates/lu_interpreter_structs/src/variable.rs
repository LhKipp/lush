use std::{rc::Rc, sync::Arc};

use lu_error::{lu_source_code_item, LuResult, SourceCodeItem};
use lu_syntax_elements::constants::IN_ARG_NAME;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::{Command, CommandCollection, Strct, Value};

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize, Hash)]
pub enum VarAttributes {
    EnvVar,
}

#[derive(Clone, Debug, Eq, PartialEq, new, Serialize, Deserialize, Hash)]
pub struct Variable {
    /// The name of the variable
    pub name: String,
    /// The evaluation value of this variable, Value::Nil in other stages of interpretation
    pub val: Value,
    pub decl: SourceCodeItem,
    #[new(default)]
    pub attrs: Vec<VarAttributes>,
}

impl Variable {
    pub fn new_nil(name: String, decl: SourceCodeItem) -> Self {
        Variable::new(name, Value::Nil, decl)
    }
    pub fn new_func(func: Rc<dyn Command>) -> Variable {
        // TODO better decl here
        let decl = func.signature().decl.clone();
        Variable::new(func.name().to_string(), Value::new_func(func), decl)
    }

    pub fn new_func_collection(funcs: Vec<Rc<dyn Command>>) -> Variable {
        // TODO better decl here
        let collection = CommandCollection::new(funcs);
        let name = collection.name();
        let decl = collection.pseudo_decl();
        Variable::new(name.to_string(), Value::CommandCollection(collection), decl)
    }

    pub fn new_strct_decl(strct: Strct) -> Variable {
        let decl = strct.decl.clone();
        Variable::new(strct.name.clone(), Value::new_strct_decl(strct), decl)
    }

    pub fn new_strct_decl_arc(strct: Arc<RwLock<Strct>>) -> Variable {
        let name = strct.read().name.clone();
        let decl = strct.read().decl.clone();
        Variable::new(name, Value::StrctDecl(strct), decl)
    }

    pub fn new_in(val: Value, decl: SourceCodeItem) -> Self {
        Self::new(IN_ARG_NAME.to_string(), val, decl)
    }
    pub fn new_args(val: Value) -> Self {
        Self::new("args".into(), val, lu_source_code_item!(-1).into())
    }

    pub fn set_val(&mut self, val: Value) -> LuResult<()> {
        if self.attrs.contains(&VarAttributes::EnvVar) {
            std::env::set_var(&self.name, val.to_string());
        }

        self.val = val;
        Ok(())
    }
}
