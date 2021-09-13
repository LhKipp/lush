use log::debug;
use lu_error::LuErr;
use std::fmt::Debug;

mod block_stmt;
mod resolve;
mod source_file;
mod ty_var;
mod value_type;

pub use resolve::*;
pub use value_type::{CustomType, ValueType};

pub struct TypeChecker {
    pub errors: Vec<LuErr>,
}

#[derive(Clone, Debug)]
pub enum TypeCheckArg {}

pub trait TypeCheck: Debug {
    /// typecheck the AST-Node/Token given the ty_state.
    fn do_typecheck(&self, args: &[TypeCheckArg], ty_state: &mut TypeChecker);

    fn typecheck(&self, ty_state: &mut TypeChecker) {
        self.typecheck_with_args(&[], ty_state)
    }

    fn typecheck_with_args(&self, args: &[TypeCheckArg], ty_state: &mut TypeChecker) {
        debug!("Typechecking: {:?}({:?})", self, args);
        let result = self.do_typecheck(args, ty_state);
        debug!(
            "Result of Typechecking: {:?}({:?}): {:?}",
            self,
            args,
            // TODO better debug stmt
            ty_state.errors.is_empty()
        );
        result
    }
}
