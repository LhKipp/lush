use bimap::BiHashMap;
use log::debug;
use lu_error::{AstErr, LuErr, LuResults};
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::{CmdStmtNode, SourceFileNode};
use lu_syntax::AstNode;
use lu_value::Value;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};
use std::{collections::HashMap, fmt::Debug};

use crate::{visit_arg::VisitArg, FlagSignature, Resolver, Scope, ValueType, Variable};
use crate::{Command, RunExternalCmd, Signature, ValueTypeErr, VarDeclNode};

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
    tc_table: BiHashMap<Variable, TcKey>,
    /// TcKey to TcFunc
    tc_func_table: HashMap<TcKey, TcFunc>,

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

        // If other is a func, we need to also equate the inner func_keys
        // We do so by inserting cloning and reinserting the tc_func
        if let Some(tc_func) = self.tc_func_table.get(&equate_with).cloned() {
            self.tc_func_table.insert(key, tc_func);
        }
        key
    }

    pub(crate) fn new_term_key_concretiziesd(
        &mut self,
        term: SourceCodeItem,
        ty: ValueType,
    ) -> TcKey {
        if let Some(func_ty) = ty.as_func() {
            // new key is a func and needs to be inserted like that
            let tc_func = TcFunc::from_signature(&*func_ty, self); // Generate func first
            self.new_term_key_equated(term, tc_func.self_key) // Set term equal to func
        } else {
            let key = self.new_term_key(term);
            let res = self.checker.impose(key.concretizes_explicit(ty));
            self.handle_tc_result(res);
            key
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

    /// Get the SourceCodeItem behind the key
    pub(crate) fn get_item_of<'a>(&'a self, key: &TcKey) -> &'a SourceCodeItem {
        self.tc_expr_table.get(key).unwrap()
    }

    /// Returns the key of the var. If the var is not in scope, it will record an error and return
    /// None
    // TODO the interface of this func is horrible.
    pub(crate) fn expect_key_of_var(
        &mut self,
        (var_name, var_name_usage): (String, SourceCodeItem),
    ) -> TcKey {
        if let Some(var) = self.scope.find_var(&var_name).cloned() {
            if let Some(var_key) = self.tc_table.get_by_left(&var) {
                *var_key
            } else {
                // Var is in scope, but doesn't have a tc_key yet (might be func or something else)
                debug!("Found var {}, which has no tc_key yet", var_name);
                if let Some(callable) = var.val_as_callable() {
                    debug!(
                        "First time usage of func {}. Inserting new tc_func.",
                        var_name
                    );
                    let tc_func = TcFunc::from_signature(callable.signature(), self);
                    self.tc_table.insert(var.clone(), tc_func.self_key.clone());
                    tc_func.self_key
                } else {
                    panic!("Var is not present, but not func: {}", var_name)
                }
            }
        } else {
            // TODO move this error generation into resolve? or somewhere else?
            self.errors
                .push(AstErr::VarNotInScope(var_name_usage.clone()).into());
            // var not present. We provide a new term key and keep going
            // TODO should we pass a decl here?
            let var = Variable::new(
                var_name.to_string(),
                Value::Nil,
                VarDeclNode::ErrorUsage(var_name_usage.clone()),
            );
            let key = self.new_term_key(var_name_usage);
            self.scope.cur_mut_frame().insert_var(var.clone());
            self.tc_table.insert(var, key);

            key
        }
    }

    pub(crate) fn find_callable(
        &mut self,
        possibl_longest_name: &[String],
        caller_node: &CmdStmtNode,
    ) -> Option<(usize, TcFunc)> {
        if let Some((name_args_split_i, var)) = self
            .scope
            .find_var_with_longest_match(&possibl_longest_name)
            .map(|(i, var)| (i, var.clone()))
        {
            // Lets assume var is a cmd here (error checking is below)
            // The cmd might not yet been inserted into the tables, as it is the first
            // usage. Therefore we can't assume to find the key here.
            let var_name = possibl_longest_name[0..name_args_split_i].join(" ");
            let var_key = self.expect_key_of_var((var_name.clone(), caller_node.to_item()));

            if let Some(called_func) = self.tc_func_table.get(&var_key) {
                // The variable is already inserted as a TcFunc
                Some((name_args_split_i, called_func.clone()))
            } else {
                // We have found such a var, but its not a function
                // This error should be catched more elaborated in special check for this
                self.errors.push(
                    TyErr::VarExpectedToBeFunc {
                        // TODO make var.decl not optional and use it here
                        var_decl: var.decl.to_item(),
                        var_usage: caller_node.to_item(),
                    }
                    .into(),
                );
                None
            }
        } else {
            // Called cmd is not found --> It's prob an external cmd
            let cmd_node = caller_node.clone();
            let cmd_name = possibl_longest_name[0].clone();
            Some((
                1,
                TcFunc::from_signature(&RunExternalCmd::new(cmd_node, cmd_name).signature(), self),
            ))
        }
    }

    fn get_expr_of(&self, key: TcKey) -> &SourceCodeItem {
        self.tc_expr_table
            .get(&key)
            .expect("Key is always inserted in expr-table")
    }

    fn insert_var(&mut self, var: Variable, key: TcKey) -> () {
        self.scope.cur_mut_frame().insert_var(var.clone());
        self.tc_table.insert(var, key);
    }

    fn get_tc_func(&self, key: &TcKey) -> Option<&TcFunc> {
        self.tc_func_table.get(key)
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
    /// Key of this func decl. (used to get SourceCodeItem from tc_expr_table)
    self_key: TcKey,

    in_ty: TcKey,
    ret_ty: TcKey,
    args_ty: Vec<TcKey>,
    var_arg_ty: Option<TcKey>,
    flags_ty: HashMap<FlagSignature, TcKey>,
}

impl TcFunc {
    /// generate a TcFunc from func. Each arg / flag / in / ret type of the func
    /// will be inserted as a seperate pseudo variable
    pub fn from_signature(sign: &Signature, ty_checker: &mut TypeChecker) -> Self {
        debug!("Generating TcFunc for Signature: {:?}", sign);
        let self_key = ty_checker.new_term_key(sign.decl.clone());

        let in_ty = ty_checker
            .new_term_key_concretiziesd(sign.in_type.decl.clone(), sign.in_type.type_.clone());

        let ret_ty = ty_checker
            .new_term_key_concretiziesd(sign.ret_type.decl.clone(), sign.ret_type.type_.clone());

        let var_arg_ty = sign
            .var_arg
            .as_ref()
            .map(|var_arg_sign| (var_arg_sign.decl.clone(), var_arg_sign.type_.clone()))
            .map(|(decl, ty)| ty_checker.new_term_key_concretiziesd(decl, ty))
            .clone();

        let args_ty = sign
            .args
            .iter()
            .map(|arg_sign| {
                ty_checker.new_term_key_concretiziesd(arg_sign.decl.clone(), arg_sign.type_.clone())
            })
            .collect();

        let ty_func = Self {
            self_key,
            in_ty,
            ret_ty,
            args_ty,
            var_arg_ty,
            // TODO gen flags tc keys
            flags_ty: HashMap::new(),
        };

        ty_checker
            .tc_func_table
            .insert(ty_func.self_key.clone(), ty_func.clone());

        ty_func
    }

    fn same_arity_as(&self, other: &TcFunc) -> bool {
        match (self.var_arg_ty, other.var_arg_ty) {
            (None, None) => self.args_ty.len() == other.args_ty.len(),
            // case self.args_ty.len == other.args_ty.len:
            //      works, as both expect same arg count
            // case self.args_ty.len > other.args_ty.len:
            //      works, as other args can be filled up in var_arg
            // case self.args_ty.len < other.args_ty.len:
            //      doesn't work, as (other.args_ty.len - self.args_ty.len) to many args for self
            (None, Some(_)) => self.args_ty.len() >= other.args_ty.len(),
            // See above
            (Some(_), None) => self.args_ty.len() <= other.args_ty.len(),
            (Some(_), Some(_)) => true,
        }
    }

    // TODO return Vec<Constraint> when constraint is pub
    fn equate_with(&self, other: &TcFunc, ty_state: &mut TypeChecker) {
        assert!(
            self.same_arity_as(other),
            "Should only equate fns with same arity???"
        );
        let in_ret_constr = [
            self.in_ty.equate_with(other.in_ty),
            self.ret_ty.equate_with(other.ret_ty),
        ];
        let self_args_ty_iter = self.args_ty.iter().chain(self.var_arg_ty.as_ref());
        let other_args_ty_iter = other.args_ty.iter().chain(other.var_arg_ty.as_ref());
        let args_constr = itertools::zip(self_args_ty_iter, other_args_ty_iter)
            .map(|(self_arg_key, other_arg_key)| self_arg_key.equate_with(*other_arg_key))
            .chain(in_ret_constr);

        for constr in args_constr {
            let res = ty_state.checker.impose(constr);
            ty_state.handle_tc_result(res);
        }
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
