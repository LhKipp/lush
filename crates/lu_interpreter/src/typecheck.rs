use bimap::BiHashMap;
use itertools::Itertools;
use log::{debug, warn};
use lu_error::{AstErr, LuErr, LuResults};
use lu_error::{SourceCodeItem, TyErr};
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::{CmdStmtNode, SourceFileNode};
use lu_syntax::AstNode;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};
use std::collections::hash_map::Entry;
use std::iter;
use std::{collections::HashMap, fmt::Debug};

use crate::{visit_arg::VisitArg, FlagSignature, Resolver, Scope, ValueType, Variable};
use crate::{Command, RunExternalCmd, Signature, Strct, ValueTypeErr};

mod block_stmt;
mod cmd_stmt;
mod expr;
mod fn_stmt;
mod let_stmt;
mod piped_cmds_stmt;
mod ret_stmt;
mod source_file;
mod statement;
mod test;
mod value_path_expr;

pub struct TyCheckState {
    /// Input from previous stage
    pub resolve: Resolver,

    /// A TcKey (TermCheckKey) always refers to a node in the ast
    // We keep track of the node for error formatting reasons. Therefore a SourceCodeItem
    // is enough
    tc_expr_table: HashMap<TcKey, SourceCodeItem>,
    /// Variable to tckey (for simple variables)
    tc_var_table: BiHashMap<Variable, TcKey>,
    /// TcKey to TcFunc
    tc_func_table: HashMap<TcKey, TcFunc>,
    /// TcKey to TcStrct
    tc_strct_table: HashMap<TcKey, TcStrct>,
    /// TcKey to Inner Tc of Array
    tc_array_table: HashMap<TcKey, TcKey>,
    /// TcKey to Generic name
    tc_generic_table: HashMap<TcKey, String>,

    /// Final result of typechecking
    pub ty_table: HashMap<TcKey, ValueType>,

    pub errors: Vec<LuErr>,

    /// The final result of this ty
    pub result: Option<ValueType>,

    scope: Scope<Variable>,
    checker: VarlessTypeChecker<ValueType>,
}

impl TyCheckState {
    pub fn new(resolve: Resolver) -> Self {
        let scope = resolve.scope.lock().clone();
        Self {
            resolve,
            scope,
            checker: VarlessTypeChecker::new(),
            errors: Vec::new(),
            tc_var_table: BiHashMap::new(),
            tc_expr_table: HashMap::new(),
            tc_func_table: HashMap::new(),
            tc_strct_table: HashMap::new(),
            tc_generic_table: HashMap::new(),
            tc_array_table: HashMap::new(),
            ty_table: HashMap::new(),
            result: None,
        }
    }

    pub fn typecheck(&mut self) {
        let source_file = self
            .resolve
            .get_start_parse()
            .cast::<SourceFileNode>()
            .unwrap();
        let source_f_path = self.resolve.get_start_parse().source.path.clone();

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
        self.equate_keys(key, equate_with);
        key
    }

    pub(crate) fn new_term_key_concretiziesd(
        &mut self,
        term: SourceCodeItem,
        ty: ValueType,
    ) -> TcKey {
        let new_key = self.new_term_key(term);
        self.concretizes_key(new_key, ty);
        new_key
    }

    pub(crate) fn equate_keys(&mut self, key1: TcKey, key2: TcKey) {
        debug!(
            "Equating keys: {:?} {:?}",
            self.get_item_of(&key1),
            self.get_item_of(&key2)
        );
        // Check whether both are arrays
        if let (Some(key1_arr_inner_tc), Some(key2_arr_inner_tc)) = (
            self.get_arr_inner_tc(&key1).cloned(),
            self.get_arr_inner_tc(&key2).cloned(),
        ) {
            self.equate_keys(key1_arr_inner_tc, key2_arr_inner_tc);
            return; // No more work to do
        }

        // Check whether both are funcs
        if let (Some(key1_func_tc), Some(key2_func_tc)) = (
            self.get_tc_func(&key1).cloned(),
            self.get_tc_func(&key2).cloned(),
        ) {
            key1_func_tc.equate_with(&key2_func_tc, self);
            return; // No more work to do
        }

        // Both are some atomic tys. Simple ty check is enough
        let res = key1.equate_with(key2);
        let res = self.checker.impose(res);
        if self.handle_tc_result(res) {
            return; // error no more equation
        }

        // If other is a func, we need to also equate the inner func_keys
        // We do so by inserting cloning and reinserting the tc_func
        if let Some(tc_func) = self.tc_func_table.get(&key2).cloned() {
            self.tc_func_table.insert(key1, tc_func);
        } else if let Some(tc_func) = self.tc_func_table.get(&key1).cloned() {
            self.tc_func_table.insert(key2, tc_func);
        } else if let Some(tc_strct) = self.tc_strct_table.get(&key2).cloned() {
            self.tc_strct_table.insert(key1, tc_strct);
        } else if let Some(tc_strct) = self.tc_strct_table.get(&key1).cloned() {
            self.tc_strct_table.insert(key2, tc_strct);
        } else if let Some(tc_array) = self.tc_array_table.get(&key2).cloned() {
            self.tc_array_table.insert(key1, tc_array);
        } else if let Some(tc_array) = self.tc_array_table.get(&key1).cloned() {
            self.tc_array_table.insert(key2, tc_array);
        }
    }

    fn concretizes_key(&mut self, key: TcKey, ty: ValueType) {
        if let Some(func_ty) = ty.as_func() {
            let tc_func = TcFunc::from_signature(&*func_ty, self); // Generate func first
            self.equate_keys(key, tc_func.self_key) // Set term equal to func
        } else if let Some(generic_name) = ty.as_generic() {
            self.tc_generic_table.insert(key, generic_name.clone()); // No further concretization needed
        } else if let Some((inner_ty, inner_ty_decl)) = ty.as_array() {
            let inner_ty_key =
                self.new_term_key_concretiziesd(inner_ty_decl.clone(), *inner_ty.clone());
            self.tc_array_table.insert(key, inner_ty_key);

            let res = self.checker.impose(key.concretizes_explicit(ty.clone()));
            self.handle_tc_result(res);
        } else {
            warn!("strct not handled");
            let res = self.checker.impose(key.concretizes_explicit(ty));
            self.handle_tc_result(res);
        }
    }

    /// TODO pass Constraint when Constraint is pub and do impose here instead on caller side
    pub(crate) fn handle_tc_result(&mut self, res: Result<(), TcErr<ValueType>>) -> bool {
        if let Err(e) = res {
            self.handle_tc_err(e);
            true
        } else {
            false
        }
    }

    pub(crate) fn handle_tc_err(&mut self, tc_err: TcErr<ValueType>) {
        let key_to_item = |key| self.tc_expr_table.get(key).cloned();

        match tc_err {
            TcErr::KeyEquation(lhs_key, rhs_key, e) => match e {
                ValueTypeErr::NotMeetAble { lhs_ty, rhs_ty } => {
                    let lhs_decl = key_to_item(&lhs_key);
                    let rhs_decl = key_to_item(&rhs_key);
                    self.push_err(
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
                    self.push_err(
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

    pub(crate) fn as_result(self) -> LuResults<TyCheckState> {
        if self.failed() {
            Err(self.collect_all_errors())
        } else {
            Ok(self)
        }
    }

    /// Returns the key of the var (if present, (or still has to be inserted))
    fn get_key_of_var(&mut self, var_name: &str) -> Option<TcKey> {
        if let Some(var) = self.scope.find_var(var_name).cloned() {
            if let Some(var_key) = self.tc_var_table.get_by_left(&var) {
                Some(*var_key)
            } else {
                // Var is in scope, but doesn't have a tc_key yet (might be func or something else)
                debug!("Found var {}, which has no tc_key yet", var_name);
                if let Some(callable) = var.val_as_callable() {
                    debug!(
                        "First time usage of func {}. Inserting new tc_func.",
                        var_name
                    );
                    let tc_func = TcFunc::from_signature(callable.signature(), self);
                    self.tc_var_table
                        .insert(var.clone(), tc_func.self_key.clone());
                    Some(tc_func.self_key)
                } else if let Some(strct) = var.val_as_strct().cloned() {
                    debug!(
                        "First time usage of strct {}. Inserting new tc_strct.",
                        strct.name
                    );
                    let tc_strct = TcStrct::from_strct(strct, self);
                    self.tc_var_table
                        .insert(var.clone(), tc_strct.self_key.clone());
                    Some(tc_strct.self_key)
                } else {
                    panic!("Var is present, but not func: {}", var_name)
                }
            }
        } else {
            None
        }
    }

    /// Returns Some(var_key) if var is present, none otherwise (and an error will be generated)
    pub(crate) fn expect_key_from_var(
        &mut self,
        var_name: &str,
        usage: SourceCodeItem,
    ) -> Option<TcKey> {
        if let Some(var_key) = self.get_key_of_var(var_name) {
            Some(var_key)
        } else {
            self.push_err(AstErr::VarNotInScope(usage.clone()).into());
            None
        }
    }

    /// Returns the strct behind key if key is a Strct. Records an error otherwise
    /// Therefore the user does not have to handle the None case
    fn expect_strct_from_key(&mut self, key: &TcKey) -> Option<&TcStrct> {
        if !self.tc_strct_table.contains_key(key) {
            let item = self.get_item_of(key).clone();
            self.push_err(TyErr::ItemExpectedToBeStruct(item).into());
        };
        self.tc_strct_table.get(key)
    }

    /// Some if such a struct is found. None otherwise (and an error will be generated)
    pub(crate) fn expect_strct_from_usage(
        &mut self,
        name: &str,
        usage: SourceCodeItem,
    ) -> Option<&TcStrct> {
        if let Some(var_key) = self.get_key_of_var(name) {
            self.tc_strct_table.get(&var_key)
        } else {
            self.push_err(AstErr::StrctNotInScope(usage.clone()).into()); // TODO this err might already be recorded in value_type
            None
        }
    }

    /// Returns the strct behind key if key is a Strct. Records an error otherwise
    /// Therefore the user does not have to handle the None case
    fn expect_callable_from_key(&mut self, key: TcKey) -> Option<TcFunc> {
        let tc_callabl = self
            .get_tc_func(&key.clone())
            .cloned()
            .map(|tc_func| tc_func.substitute_generics(self));

        if tc_callabl.is_none() {
            let key_item = self.get_item_of(&key).clone();
            self.push_err(TyErr::ItemExpectedToBeFunc(key_item).into());
        }
        tc_callabl
    }

    /// Some if such a callable is found. None otherwise (and an error will be generated)
    fn expect_callable_from_var(
        &mut self,
        cllbl_name: &str,
        cllbl_usage: SourceCodeItem,
    ) -> Option<TcFunc> {
        if let Some(var_key) = self.expect_key_from_var(&cllbl_name, cllbl_usage) {
            self.expect_callable_from_key(var_key)
        } else {
            None
        }
    }

    /// Some for internal and external cmds
    pub(crate) fn expect_callable_with_longest_name(
        &mut self,
        possibl_longest_name: &[String],
        caller_node: &CmdStmtNode,
    ) -> Option<(usize, TcFunc)> {
        if let Some((name_args_split_i, _)) = self
            .scope
            .find_var_with_longest_match(&possibl_longest_name)
            .map(|(i, var)| (i, var.clone()))
        {
            let var_name = possibl_longest_name[0..name_args_split_i].join(" ");
            self.expect_callable_from_var(&var_name, caller_node.to_item())
                .map(|callabl| (name_args_split_i, callabl.clone()))
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

    /// Returns the inner_ty key behind key if key is a array. Records an error otherwise
    /// Therefore the user does not have to handle the None case
    fn expect_arr_inner_ty_from_key(&mut self, array_key: TcKey) -> Option<TcKey> {
        let inner_ty_key = self.get_arr_inner_tc(&array_key.clone()).cloned();

        if inner_ty_key.is_none() {
            let key_item = self.get_item_of(&array_key).clone();
            self.push_err(TyErr::ItemExpectedToBeArray(key_item).into());
        }
        inner_ty_key
    }

    /// Get the SourceCodeItem behind the key
    pub(crate) fn get_item_of(&self, key: &TcKey) -> &SourceCodeItem {
        self.tc_expr_table.get(key).unwrap()
    }

    /// Insert var var with ty ty
    fn insert_var(&mut self, var: Variable) -> TcKey {
        self.scope.cur_mut_frame().insert_var(var.clone());
        let key = self.new_term_key(var.decl.to_item());
        self.tc_var_table.insert(var, key.clone());
        key
    }

    fn get_tc_func(&self, key: &TcKey) -> Option<&TcFunc> {
        self.tc_func_table.get(key)
    }

    fn get_tc_generic(&self, key: &TcKey) -> Option<&String> {
        self.tc_generic_table.get(key)
    }

    fn get_arr_inner_tc(&self, key: &TcKey) -> Option<&TcKey> {
        self.tc_array_table.get(key)
    }
}

impl PipelineStage for TyCheckState {
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
#[derive(Debug, Clone, new)]
pub struct TcStrct {
    /// Key of this strct decl. (used to get SourceCodeItem from tc_expr_table)
    self_key: TcKey,
    /// Always sorted by field name
    field_keys: Vec<(String, TcKey)>,
}

impl TcStrct {
    pub fn from_strct(strct: Strct, ty_state: &mut TyCheckState) -> Self {
        debug!("Generating TcStrct for Struct: {:?}", strct);
        let field_keys = strct
            .fields
            .iter()
            .map(|field| {
                (
                    field.name.clone(),
                    ty_state.new_term_key_concretiziesd(field.decl.clone(), field.ty.clone()),
                )
            })
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();

        let self_key =
            ty_state.new_term_key_concretiziesd(strct.decl.clone(), ValueType::new_strct(strct));

        let tc_strct = Self {
            self_key,
            field_keys,
        };

        ty_state
            .tc_strct_table
            .insert(tc_strct.self_key.clone(), tc_strct.clone());

        // TODO assert fields are sorted
        // assert!(
        //     tc_strct.field_keys.is_sorted_by_key(|(name, _)| name),
        //     "Fields must be sorted by name"
        // );

        tc_strct
    }
}

#[derive(Debug, Clone)]
pub struct TcFunc {
    /// Key of this func decl. (used to get SourceCodeItem from tc_expr_table)
    self_key: TcKey,

    in_key: TcKey,
    ret_key: TcKey,
    args_keys: Vec<TcKey>,
    var_arg_key: Option<TcKey>,
    flags_keys: HashMap<FlagSignature, TcKey>,
}

impl TcFunc {
    #[allow(unused)]
    pub fn all_keys_iter(&self) -> impl Iterator<Item = TcKey> + '_ {
        iter::once(self.in_key.clone())
            .chain(iter::once(self.ret_key.clone()))
            .chain(self.args_keys.clone())
            .chain(self.var_arg_key)
    }

    pub(crate) fn substitute_generics(self, ty_state: &mut TyCheckState) -> TcFunc {
        let mut generics_key = HashMap::<String, TcKey>::new();
        self.substitute_generics_rec(&mut generics_key, ty_state)
    }

    fn substitute_generics_rec(
        mut self,
        seen_generics: &mut HashMap<String, TcKey>,
        ty_state: &mut TyCheckState,
    ) -> TcFunc {
        let mut subst_generic_key = |key: TcKey| {
            if let Some(generic_name) = ty_state.get_tc_generic(&key).cloned() {
                let key_item = ty_state.get_item_of(&key).clone();
                // Generic key which needs to be substituted
                let generic_key = ty_state.new_term_key(key_item.clone());
                match seen_generics.entry(generic_name.clone()) {
                    Entry::Occupied(already_inserted_key) => {
                        let already_inserted_generic_item =
                            ty_state.get_item_of(already_inserted_key.get());
                        debug!(
                            "Unifying {:?} with other generic {:?}",
                            key_item, already_inserted_generic_item
                        );
                        ty_state.equate_keys(generic_key, already_inserted_key.get().clone());
                    }
                    Entry::Vacant(v) => {
                        debug!(
                            "Found generic ty: {:?} for first time and substituted it.",
                            key_item
                        );
                        v.insert(generic_key);
                    }
                };
                generic_key
            } else if let Some(tc_func) = ty_state.get_tc_func(&key).cloned() {
                // Recurse into tc_func which also needs to be substituted
                let key = tc_func.self_key.clone();
                tc_func.substitute_generics_rec(seen_generics, ty_state);
                key
            } else {
                warn!("NOT RECURSING INTO ARRAY. NOT IMPL YET");
                key
            }
        };

        self.in_key = subst_generic_key(self.in_key);
        self.ret_key = subst_generic_key(self.ret_key);
        if let Some(var_arg_key) = &mut self.var_arg_key {
            *var_arg_key = subst_generic_key(*var_arg_key);
        }
        for arg_key in self.args_keys.iter_mut() {
            *arg_key = subst_generic_key(*arg_key)
        }
        // TODO handle flags

        self
    }

    /// generate a TcFunc from func. Each arg / flag / in / ret type of the func
    /// will be inserted as a seperate pseudo variable
    pub fn from_signature(sign: &Signature, ty_state: &mut TyCheckState) -> Self {
        debug!("Generating TcFunc for Signature: {:?}", sign);
        let self_key = ty_state.new_term_key(sign.decl.clone()); // TODO shouldn't self key be concretizied to be fn???

        let in_key =
            ty_state.new_term_key_concretiziesd(sign.in_arg.decl.clone(), sign.in_arg.ty.clone());

        let ret_key =
            ty_state.new_term_key_concretiziesd(sign.ret_arg.decl.clone(), sign.ret_arg.ty.clone());

        let var_arg_key = sign
            .var_arg
            .as_ref()
            .map(|var_arg_sign| (var_arg_sign.decl.clone(), var_arg_sign.ty.clone()))
            .map(|(decl, ty)| ty_state.new_term_key_concretiziesd(decl, ty))
            .clone();

        let args_keys = sign
            .args
            .iter()
            .map(|arg_sign| {
                ty_state.new_term_key_concretiziesd(arg_sign.decl.clone(), arg_sign.ty.clone())
            })
            .collect();

        let ty_func = Self {
            self_key,
            in_key,
            ret_key,
            args_keys,
            var_arg_key,
            // TODO gen flags tc keys
            flags_keys: HashMap::new(),
        };

        ty_state
            .tc_func_table
            .insert(ty_func.self_key.clone(), ty_func.clone());

        ty_func
    }

    fn same_arity_as(&self, other: &TcFunc) -> bool {
        match (self.var_arg_key, other.var_arg_key) {
            (None, None) => self.args_keys.len() == other.args_keys.len(),
            // case self.args_ty.len == other.args_ty.len:
            //      works, as both expect same arg count
            // case self.args_ty.len > other.args_ty.len:
            //      works, as other args can be filled up in var_arg
            // case self.args_ty.len < other.args_ty.len:
            //      doesn't work, as (other.args_ty.len - self.args_ty.len) to many args for self
            (None, Some(_)) => self.args_keys.len() >= other.args_keys.len(),
            // See above
            (Some(_), None) => self.args_keys.len() <= other.args_keys.len(),
            (Some(_), Some(_)) => true,
        }
    }

    // TODO return Vec<Constraint> when constraint is pub
    fn equate_with(&self, other: &TcFunc, ty_state: &mut TyCheckState) {
        assert!(
            self.same_arity_as(other),
            "Should only equate fns with same arity???"
        );
        let in_ret_constr = [
            self.in_key.equate_with(other.in_key),
            self.ret_key.equate_with(other.ret_key),
        ];
        let self_args_key_iter = self.args_keys.iter().chain(self.var_arg_key.as_ref());
        let other_args_key_iter = other.args_keys.iter().chain(other.var_arg_key.as_ref());
        let args_constr = itertools::zip(self_args_key_iter, other_args_key_iter)
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
    fn do_typecheck(&self, args: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey>;

    fn typecheck(&self, ty_state: &mut TyCheckState) -> Option<TcKey> {
        self.typecheck_with_args(&[], ty_state)
    }

    fn typecheck_with_args(
        &self,
        args: &[TypeCheckArg],
        ty_state: &mut TyCheckState,
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
    fn do_typecheck(&self, args: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
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
