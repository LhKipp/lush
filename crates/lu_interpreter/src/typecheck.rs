use bimap::BiHashMap;
use log::debug;
use lu_error::{LuErr, LuResults};
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::ErrorContainer;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::SourceFileNode;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};
use std::{collections::HashMap, fmt::Debug};

use crate::ValueTypeErr;
use crate::{visit_arg::VisitArg, FlagSignature, Function, Resolver, Scope, ValueType, Variable};

mod block_stmt;
mod expr;
mod let_stmt;
mod source_file;
mod statement;

pub struct TypeChecker {
    /// Input from previous stage
    pub resolve: Resolver,

    /// A TcKey (TermCheckKey) always refers to a node in the ast
    tc_expr_table: HashMap<TcKey, SourceCodeItem>,
    /// Variable to tckey (for simple variables)
    pub tc_table: BiHashMap<Variable, TcKey>,
    /// Variable to tcfunc (for func variables)
    pub tc_func_table: HashMap<Variable, TcFunc>,

    /// Final result of typechecking
    pub ty_table: HashMap<TcKey, ValueType>,

    pub errors: Vec<LuErr>,

    scope: Scope<Variable>,
    checker: VarlessTypeChecker<ValueType>,
}

impl TypeChecker {
    pub fn new(resolve: Resolver) -> Self {
        let scope = resolve.scope.lock().clone();
        Self {
            resolve,
            scope,
            checker: VarlessTypeChecker::new(),
            errors: Vec::new(),
            tc_table: BiHashMap::new(),
            tc_expr_table: HashMap::new(),
            tc_func_table: HashMap::new(),
            ty_table: HashMap::new(),
        }
    }

    pub fn all_errors(&self) -> Vec<LuErr> {
        let mut errs = self.resolve.all_errors();
        errs.extend(self.errors.clone());
        errs
    }

    pub fn typecheck(&mut self) {
        let source_file = self.resolve.parse.cast::<SourceFileNode>().unwrap();
        let source_f_path = self.resolve.parse.source.path.clone();

        source_file.typecheck_with_args(
            &[TypeCheckArg::Arg(VisitArg::SourceFilePath(source_f_path))],
            self,
        );

        match self.checker.clone().type_check() {
            Ok(t) => self.ty_table = t,
            Err(e) => self.handle_tc_err(e),
        }
    }

    pub(crate) fn new_term_key(&mut self, term: SourceCodeItem) -> TcKey {
        let key = self.checker.new_term_key();
        self.tc_expr_table.insert(key, term);
        key
    }

    /// TODO pass Constraint when Constraint is pub and do impose here instead on caller side
    pub(crate) fn handle_tc_result(&mut self, res: Result<(), TcErr<ValueType>>) {
        if let Err(e) = res {
            self.handle_tc_err(e)
        }
    }

    pub(crate) fn handle_tc_err(&mut self, tc_err: TcErr<ValueType>) {
        let key_to_item = |key| self.tc_expr_table.get(key).cloned();

        match tc_err {
            TcErr::KeyEquation(lhs_key, rhs_key, e) => match e {
                ValueTypeErr::NotMeetAble { lhs_ty, rhs_ty } => {
                    let lhs_decl = key_to_item(&lhs_key);
                    let rhs_decl = key_to_item(&rhs_key);
                    self.errors.push(
                        TyErr::TypesNotEqual {
                            lhs_decl,
                            lhs_ty: lhs_ty.to_string(),
                            rhs_decl,
                            rhs_ty: rhs_ty.to_string(),
                        }
                        .into(),
                    )
                }
                _ => unreachable!(),
            },
            TcErr::Bound(k, o, e) => match e {
                ValueTypeErr::NotMeetAble { rhs_ty, lhs_ty } => {
                    let var_decl = key_to_item(&k);
                    let other_decl = o.as_ref().map(key_to_item).flatten();
                    self.errors.push(
                        TyErr::TypesNotEqual {
                            lhs_decl: var_decl,
                            lhs_ty: lhs_ty.to_string(),
                            rhs_decl: other_decl,
                            rhs_ty: rhs_ty.to_string(),
                        }
                        .into(),
                    )
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub(crate) fn any_failed(&self) -> bool {
        self.resolve.any_failed() || !self.errors.is_empty()
    }

    pub(crate) fn as_result(self) -> LuResults<TypeChecker> {
        if self.any_failed() {
            Err(self.all_errors())
        } else {
            Ok(self)
        }
    }
}

impl PipelineStage for TypeChecker {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.resolve)
    }
}

impl ErrorContainer for TypeChecker {
    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}

#[derive(Debug, Clone)]
pub enum TcEntry {
    Func(TcFunc),
    Var(TcKey),
}

#[derive(Debug, Clone)]
pub struct TcFunc {
    in_ty: Option<TcKey>,
    ret_ty: Option<TcKey>,
    args_ty: Vec<TcKey>,
    var_arg_ty: Option<TcKey>,
    flags_ty: HashMap<FlagSignature, TcKey>,
}

impl TcFunc {
    /// generate a TcFunc from func. Each arg / flag / in / ret type of the func
    /// will be inserted as a seperate pseudo variable
    pub fn from_func(func: Function, ty_checker: &mut TypeChecker) -> Self {
        fn tc_key_with_opt_type(
            var: Variable,
            bound: &Option<ValueType>,
            ty_checker: &mut TypeChecker,
        ) -> TcKey {
            let tc_key = ty_checker.checker.new_term_key();
            if let Some(type_decl) = bound {
                ty_checker
                    .checker
                    .impose(tc_key.concretizes_explicit(type_decl.clone()))
                    .unwrap();
            }
            // TODO we insert the var into the tc_table but not into the scope
            // therefore we shouldn't violate scoping rules here, about variable visibility
            // The passed var is also more like a pseudo variable, so that we can later on
            // refer to the declaration via a unified interface...
            // I think, what is being done here is safe, but needs some more thought
            ty_checker.tc_table.insert(var, tc_key.clone());
            tc_key
        }

        macro_rules! gen_key_of {
            ($t:ident) => {{
                let ty = $t.type_.clone();
                let var: Variable = $t.into();
                tc_key_with_opt_type(var, &ty, ty_checker)
            }};
        }
        // TODO write macro that does the transformation
        let var_arg_ty = func
            .signature
            .var_arg
            .clone()
            .map(|var_arg_sign| gen_key_of!(var_arg_sign));
        let in_ty = func
            .signature
            .in_type
            .clone()
            .map(|in_sign| gen_key_of!(in_sign));
        let ret_ty = func
            .signature
            .ret_type
            .clone()
            .map(|ret_sign| gen_key_of!(ret_sign));
        let args_ty = func
            .signature
            .args
            .into_iter()
            .map(|arg_sign| gen_key_of!(arg_sign))
            .collect();

        let ty_func = Self {
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

#[derive(Clone, Debug)]
pub enum TypeCheckArg {
    Arg(VisitArg),
}

pub trait TypeCheck: Debug {
    /// typecheck the AST-Node/Token given the ty_state.
    /// Returns if successful the infered type, () otherwise (Errors will be accumulated in the type checker
    /// itself).
    // (A statement which does not have a return value can never be the rhs of something expecting
    // a type. This kind of error would be catched at parsing level (e.G. let x = let y = 1))
    fn do_typecheck(&self, args: &[TypeCheckArg], ty_state: &mut TypeChecker) -> Option<TcKey>;

    fn typecheck(&self, ty_state: &mut TypeChecker) -> Option<TcKey> {
        self.typecheck_with_args(&[], ty_state)
    }

    fn typecheck_with_args(
        &self,
        args: &[TypeCheckArg],
        ty_state: &mut TypeChecker,
    ) -> Option<TcKey> {
        debug!("Typechecking: {:?}({:?})", self, args);
        let result = self.do_typecheck(args, ty_state);
        debug!(
            "Result of Typechecking: {:?}({:?}): {:?}",
            self,
            args,
            // TODO better debug stmt
            result,
        );
        result
    }
}

impl<T: TypeCheck> TypeCheck for Option<T> {
    fn do_typecheck(&self, args: &[TypeCheckArg], ty_state: &mut TypeChecker) -> Option<TcKey> {
        match self {
            Some(n) => n.typecheck_with_args(args, ty_state),
            None => {
                // We have an incomplete Ast here. We should not generate an error
                let key = ty_state.checker.new_term_key();
                // TODO check whether Error is fine here. Should be as error should not generate
                // further erorrs
                ty_state
                    .checker
                    .impose(key.concretizes_explicit(ValueType::Error))
                    .expect("New key can always be conretizised");
                Some(key)
            }
        }
    }
}
