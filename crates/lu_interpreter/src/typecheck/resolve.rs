#![allow(unused_imports)]
mod block_stmt;
mod source_file;

use std::fmt::Debug;
use std::{collections::HashMap, rc::Rc};

use log::debug;
use lu_error::{LuErr, TyErr};
use lu_syntax::ast::{CmdStmtNode, ValuePathExprNode};
use lu_syntax_elements::BlockType;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};

use crate::{FlagSignature, Function, Scope, Variable};

use super::{value_type::CustomType, ValueType};

pub trait Resolve: Debug {
    fn do_resolve_dependant_names(&self, args: &[ResolveArg], resolver: &mut Resolver);

    fn resolve_dependant_names(&self, resolver: &mut Resolver) {
        self.resolve_dependant_names_with_args(&[], resolver)
    }

    fn resolve_dependant_names_with_args(&self, args: &[ResolveArg], resolver: &mut Resolver) {
        debug!("Resolving dependant names in: {:?}({:?})", self, args);
        let result = self.do_resolve_dependant_names(args, resolver);
        debug!(
            "Result of resolving dependant names: {:?}({:?}): {:?}",
            self,
            args,
            // TODO better debug stmt
            resolver
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
    pub fn from_func(func: Function, resolver: &mut Resolver) -> Self {
        fn tc_key_with_opt_type(bound: &Option<ValueType>, resolver: &mut Resolver) -> TcKey {
            let tc_key = resolver.checker.new_term_key();
            if let Some(type_decl) = bound {
                resolver
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
            .map(|var_arg_sign| tc_key_with_opt_type(&var_arg_sign.type_, resolver));
        let in_ty = func
            .signature
            .in_type
            .clone()
            .map(|in_sign| tc_key_with_opt_type(&Some(in_sign), resolver));
        let ret_ty = func
            .signature
            .ret_type
            .clone()
            .map(|ret_sign| tc_key_with_opt_type(&Some(ret_sign), resolver));
        let args_ty = func
            .signature
            .args
            .iter()
            .map(|arg_sign| tc_key_with_opt_type(&arg_sign.type_, resolver))
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

#[derive(Debug, Clone)]
/// Element taking place in resolution
pub enum ResoElem {
    Func(Rc<TyFunction>),
    Var(Rc<TyVariable>),
    // TODO custom types
}

#[derive(Educe)]
#[educe(Debug)]
/// TypeChecking runs in 2 steps:
/// 1. Resolve elements
/// 2. Actual typecheck
///
/// Step 1 includes:
///     Bringing all custom types, funcs into scope
///     Building a map from string to CustomType
/// Step 2 includes:
///     actual typechecking
/// TODO better docs
pub struct Resolver {
    #[educe(Debug(ignore))]
    pub scope: Scope<ResoElem>,
    /// Table from $var -> TcKey
    pub var_access_table: HashMap<ValuePathExprNode, Rc<TyVariable>>,
    /// Table from cmd_call to
    pub cmd_stmts: HashMap<CmdStmtNode, TcKey>,
    // TODO custom types need to be in the scope
    pub custom_types: HashMap<String, CustomType>,
    pub errors: Vec<LuErr>,

    pub checker: VarlessTypeChecker<ValueType>,
}

#[derive(Clone, Debug)]
pub enum ResolveArg {
    Dummy,
    BlockTypeArg(BlockType),
}

impl Resolver {
    pub(crate) fn ok_or_record_err(&mut self, ty: Result<ValueType, LuErr>) -> ValueType {
        match ty {
            Ok(t) => t,
            Err(e) => {
                self.errors.push(e);
                ValueType::Error
            }
        }
    }
}

// fn tc_err_to_ty_err(tc_err: TcErr<ValueType>) -> TyErr {
//     match tc_err {
//         TcErr::KeyEquation(_, _, _) => todo!(),
//         TcErr::Bound(_, _, _) => todo!(),
//         TcErr::ChildAccessOutOfBound(_, _, _) => todo!(),
//         TcErr::ArityMismatch {
//             _key,
//             _variant,
//             _inferred_arity,
//             _reported_arity,
//         } => todo!(),
//         TcErr::Construction(_, _, _) => todo!(),
//         TcErr::ChildConstruction(_, _, _, _) => todo!(),
//     }
// }
