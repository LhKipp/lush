#![allow(unused_imports)]
#![allow(unused_variables)]
use lu_error::TyErr;
use lu_pipeline_stage::ErrorContainer;
use lu_syntax::{
    ast::{CmdStmtNode, LetStmtNode, PipedCmdsStmtNode},
    AstElement,
};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable};

impl TypeCheck for PipedCmdsStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TypeChecker,
    ) -> Option<TcKey> {
        todo!()
    }
}
