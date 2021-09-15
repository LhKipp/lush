use log::debug;
use lu_error::LuErr;
use lu_syntax::ast::SourceFileNode;
use parking_lot::Mutex;
use rusttyc::{TcKey, VarlessTypeChecker};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{FlagSignature, Function, Scope, ValueType, Variable};

mod block_stmt;
mod source_file;
mod ty_var;

pub struct TypeChecker {
    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub checker: VarlessTypeChecker<ValueType>,
    pub errors: Vec<LuErr>,
}

impl TypeChecker {
    pub fn new(scope: Arc<Mutex<Scope<Variable>>>) -> Self {
        Self {
            scope,
            checker: VarlessTypeChecker::new(),
            errors: Vec::new(),
        }
    }

    pub fn typecheck(&mut self, node: &SourceFileNode) {
        node.typecheck(self);
    }
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

#[derive(Debug, Clone)]
pub struct TyFunction {
    func: Function,
    in_ty: Option<TcKey>,
    ret_ty: Option<TcKey>,
    args_ty: Vec<TcKey>,
    var_arg_ty: Option<TcKey>,
    flags_ty: HashMap<FlagSignature, TcKey>,
}

impl TyFunction {
    pub fn from_func(func: Function, ty_checker: &mut TypeChecker) -> Self {
        fn tc_key_with_opt_type(bound: &Option<ValueType>, ty_checker: &mut TypeChecker) -> TcKey {
            let tc_key = ty_checker.checker.new_term_key();
            if let Some(type_decl) = bound {
                ty_checker
                    .checker
                    .impose(tc_key.concretizes_explicit(type_decl.clone()))
                    .unwrap();
            }
            tc_key
        }

        let var_arg_ty = func
            .signature
            .var_arg
            .clone()
            .map(|var_arg_sign| tc_key_with_opt_type(&var_arg_sign.type_, ty_checker));
        let in_ty = func
            .signature
            .in_type
            .clone()
            .map(|in_sign| tc_key_with_opt_type(&Some(in_sign), ty_checker));
        let ret_ty = func
            .signature
            .ret_type
            .clone()
            .map(|ret_sign| tc_key_with_opt_type(&Some(ret_sign), ty_checker));
        let args_ty = func
            .signature
            .args
            .iter()
            .map(|arg_sign| tc_key_with_opt_type(&arg_sign.type_, ty_checker))
            .collect();

        let ty_func = Self {
            func,
            in_ty,
            ret_ty,
            args_ty,
            var_arg_ty,
            // TODO gen flags tc keys
            flags_ty: HashMap::new(),
        };

        ty_func
    }
}

#[derive(Debug, Clone, new)]
pub struct TyVariable {
    var: Variable,
    key: TcKey,
}
