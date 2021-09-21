use bimap::BiHashMap;
use log::debug;
use lu_error::{LuErr, LuResults};
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::SourceFileNode;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};
use std::{collections::HashMap, fmt::Debug};

use crate::ValueTypeErr;
use crate::{visit_arg::VisitArg, FlagSignature, Function, Resolver, Scope, ValueType, Variable};

mod block_stmt;
mod cmd_stmt;
mod expr;
mod let_stmt;
mod piped_cmds_stmt;
mod source_file;
mod statement;
mod test;

pub struct TypeChecker {
    /// Input from previous stage
    pub resolve: Resolver,

    /// A TcKey (TermCheckKey) always refers to a node in the ast
    // We keep track of the node for error formatting reasons. Therefore a SourceCodeItem
    // is enough
    tc_expr_table: HashMap<TcKey, SourceCodeItem>,
    /// Variable to tckey (for simple variables)
    pub tc_table: BiHashMap<Variable, TcKey>,
    /// Variable to tcfunc (for func variables)
    pub tc_func_table: HashMap<Variable, TcFunc>,

    /// To not spam the tables with error keys, we keep one
    #[allow(dead_code)]
    tc_error_key: Option<TcKey>,

    /// Final result of typechecking
    pub ty_table: HashMap<TcKey, ValueType>,

    pub errors: Vec<LuErr>,

    /// The final result of this evaluator
    pub result: Option<ValueType>,

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
            tc_error_key: None,
            result: None,
        }
    }

    pub fn typecheck(&mut self) {
        let source_file = self.resolve.parse.cast::<SourceFileNode>().unwrap();
        let source_f_path = self.resolve.parse.source.path.clone();

        let ret_key = source_file.typecheck_with_args(
            &[TypeCheckArg::Arg(VisitArg::SourceFilePath(source_f_path))],
            self,
        );

        match self.checker.clone().type_check() {
            Ok(t) => {
                self.ty_table = t;
                self.result = ret_key.map(|k| self.ty_table.get(&k).unwrap().clone())
            }
            Err(e) => {
                self.handle_tc_err(e);
            }
        }
    }

    pub(crate) fn new_term_key(&mut self, term: SourceCodeItem) -> TcKey {
        let key = self.checker.new_term_key();
        self.tc_expr_table.insert(key, term);
        key
    }

    pub(crate) fn new_term_key_equated(
        &mut self,
        term: SourceCodeItem,
        equate_with: TcKey,
    ) -> TcKey {
        let key = self.new_term_key(term);
        let res = self.checker.impose(key.equate_with(equate_with));
        self.handle_tc_result(res);
        key
    }

    pub(crate) fn new_term_key_concretiziesd(
        &mut self,
        term: SourceCodeItem,
        ty: ValueType,
    ) -> TcKey {
        let key = self.new_term_key(term);
        let res = self.checker.impose(key.concretizes_explicit(ty));
        self.handle_tc_result(res);
        key
    }

    // This is prob a very bad idea
    #[allow(dead_code)]
    pub(crate) fn get_tc_error_key(&mut self) -> TcKey {
        if let Some(key) = self.tc_error_key {
            key
        } else {
            let error_key = self.checker.new_term_key();
            self.checker
                .impose(error_key.concretizes_explicit(ValueType::Error))
                .unwrap();
            self.tc_error_key = Some(error_key);
            error_key
        }
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

    pub(crate) fn as_result(self) -> LuResults<TypeChecker> {
        if self.failed() {
            Err(self.collect_all_errors())
        } else {
            Ok(self)
        }
    }

    pub fn get_item_of<'a>(&'a self, non_passed_arg: &TcKey) -> &'a SourceCodeItem {
        self.tc_expr_table.get(non_passed_arg).unwrap()
    }
}

impl PipelineStage for TypeChecker {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.resolve)
    }

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
    in_ty: TcKey,
    ret_ty: TcKey,
    args_ty: Vec<TcKey>,
    var_arg_ty: Option<TcKey>,
    flags_ty: HashMap<FlagSignature, TcKey>,
}

impl TcFunc {
    /// generate a TcFunc from func. Each arg / flag / in / ret type of the func
    /// will be inserted as a seperate pseudo variable
    pub fn from_func(func: Function, ty_checker: &mut TypeChecker) -> Self {
        // ret and in are always concretly infered.
        // if there is no decl, it's infered as AnyOf<T>

        let in_item = func.signature.in_type.decl.into_item();
        let in_ty = ty_checker.new_term_key_concretiziesd(in_item, func.signature.in_type.type_);

        let ret_item = func.signature.ret_type.decl.into_item();
        let ret_ty = ty_checker.new_term_key_concretiziesd(ret_item, func.signature.ret_type.type_);

        let var_arg_ty = func
            .signature
            .var_arg
            .map(|var_arg_sign| (var_arg_sign.decl.into_item(), var_arg_sign.type_))
            .map(|(decl, ty)| ty_checker.new_term_key_concretiziesd(decl, ty));

        let args_ty = func
            .signature
            .args
            .into_iter()
            .map(|arg_sign| {
                ty_checker.new_term_key_concretiziesd(arg_sign.decl.into_item(), arg_sign.type_)
            })
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

// TODO remove this. Its broken. Sometimes this is an error sometimes its not
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
