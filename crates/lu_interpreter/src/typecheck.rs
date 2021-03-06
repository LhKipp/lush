use bimap::BiHashMap;
use enum_as_inner::EnumAsInner;
use itertools::Itertools;
use log::{debug, trace, warn};
use lu_error::{AstErr, LuErr, LuResults};
use lu_error::{SourceCodeItem, TyErr};
use lu_interpreter_structs::prelude::VarAttributes;
use lu_interpreter_structs::{ArgSignature, Command, FlagVariant};
use lu_pipeline_stage::PipelineStage;
use parking_lot::RwLock;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::{Arc, Weak};
use std::time::Duration;
use std::{collections::HashMap, fmt::Debug};

use crate::{visit_arg::VisitArg, FlagSignature, Scope, ValueType, Variable};
use crate::{Signature, Strct, ValueTypeErr};

mod block_stmt;
mod cls_expr;
pub mod cmd_select;
mod cmd_stmt;
mod condition;
mod expr;
mod fn_cls_ty_check;
mod fn_stmt;
mod for_stmt;
mod if_stmt;
mod let_stmt;
mod math_expr;
mod piped_cmds_stmt;
mod ret_stmt;
mod source_file;
mod statement;
mod table_expr;
mod test;
mod value_path_expr;

pub struct TyCheckState {
    /// A TcKey (TermCheckKey) always refers to a node in the ast
    // We keep track of the node for error formatting reasons. Therefore a SourceCodeItem
    // is enough
    tc_expr_table: HashMap<TcKey, SourceCodeItem>,

    // TcKey to its equated keys
    tc_equated_keys: HashMap<TcKey, Vec<TcKey>>,

    /// Variable to tckey (for simple variables + strcts)
    tc_var_table: BiHashMap<Variable, TcKey>,
    /// Command to tckey
    tc_var_cmd_table: Vec<(Rc<dyn Command>, TcKey)>,

    /// TcKey to TcFunc
    tc_func_table: HashMap<TcKey, TcFunc>,
    /// TcKey to TcStrct
    tc_strct_table: HashMap<TcKey, TcStrct>,
    /// TcKey to Inner Tc of Array
    tc_array_table: HashMap<TcKey, TcKey>,
    /// TcKey to Inner Tc of Optional
    tc_optional_table: HashMap<TcKey, TcKey>,
    /// TcKey to Generic name
    tc_generic_table: HashMap<TcKey, String>,

    /// Final result of typechecking
    pub ty_table: HashMap<TcKey, ValueType>,

    pub(crate) errors: Vec<LuErr>,

    /// The final result of this ty
    pub result: Option<ValueType>,

    pub scope: Scope<Variable>,
    checker: VarlessTypeChecker<ValueType>,
}

impl TyCheckState {
    pub fn new(scope: Scope<Variable>) -> Self {
        let mut ty_state = Self {
            scope,
            checker: VarlessTypeChecker::new(),
            errors: Vec::new(),
            tc_var_table: BiHashMap::new(),
            tc_expr_table: HashMap::new(),
            tc_func_table: HashMap::new(),
            tc_strct_table: HashMap::new(),
            tc_generic_table: HashMap::new(),
            tc_array_table: HashMap::new(),
            tc_optional_table: HashMap::new(),
            ty_table: HashMap::new(),
            result: None,
            tc_var_cmd_table: Vec::new(),
            tc_equated_keys: HashMap::new(),
        };

        let vars: Vec<_> = ty_state.scope.all_vars().cloned().collect();
        for var in vars {
            if !var.attrs.contains(&VarAttributes::EnvVar) {
                continue;
            }
            trace!(
                "Inserted env var {} from start scope in tychecker",
                var.name
            );
            let key = ty_state.new_term_key(var.decl.clone());
            ty_state.tc_var_table.insert(var.clone(), key);
        }

        ty_state
    }

    pub fn typecheck(&mut self, node: impl TypeCheck) {
        let ret_key = node.typecheck(self);

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

    /// Expect to get the ty behind key. The ty behind key may not yet be inferred. In that case an
    /// error is recorded.
    /// This is an expensive operation
    #[allow(dead_code)]
    pub(crate) fn expect_ty_of_key(&mut self, key: TcKey) -> Option<ValueType> {
        if let Ok(t) = self.checker.clone().type_check() {
            if let Some(ty) = t.get(&key) {
                return Some(ty.clone());
            }
        }
        self.push_err(
            TyErr::ExpectedStmtToBeInferred {
                stmt: self.get_item_of(&key).clone(),
            }
            .into(),
        );

        None
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
        self.equate_keys_inner(key1.clone(), key2.clone());
        // After equating we have to register this fact
        for (a, b) in [(&key1, &key2), (&key2, &key1)] {
            self.tc_equated_keys
                .entry(a.clone())
                .or_default()
                .push(b.clone())
        }
    }
    pub(crate) fn equate_keys_inner(&mut self, key1: TcKey, key2: TcKey) {
        let equated_keys_with_key = |key: TcKey, ty_checker: &mut TyCheckState| -> Vec<TcKey> {
            ty_checker
                .tc_equated_keys
                .entry(key)
                .or_default()
                .iter()
                .chain(Some(&key))
                .cloned()
                .collect()
        };
        debug!(
            "Equating({}({:?}),{}({:?}))",
            self.get_item_of(&key1),
            key1,
            self.get_item_of(&key2),
            key2,
        );
        // Check whether both are arrays
        if let (Some(key1_arr_inner_tc), Some(key2_arr_inner_tc)) = (
            self.get_arr_inner_tc(&key1).cloned(),
            self.get_arr_inner_tc(&key2).cloned(),
        ) {
            self.equate_keys(key1_arr_inner_tc, key2_arr_inner_tc);
            return; // No more work to do
        }

        // Check whether both are optionals
        if let (Some(key1_arr_inner_tc), Some(key2_arr_inner_tc)) = (
            self.get_optional_inner_tc(&key1).cloned(),
            self.get_optional_inner_tc(&key2).cloned(),
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

        warn!("Not checking whether both are structs and equating all fields");

        // Both are some atomic tys. Simple ty check is enough
        let res = key1.equate_with(key2);
        let res = self.checker.impose(res);
        if self.handle_tc_result(res) {
            return; // error no more equation
        }

        // If other is a func, we need to also equate the inner func_keys
        // We do so by inserting cloning and reinserting the tc_func
        /* For Funcs */
        if let Some(tc_func) = self.tc_func_table.get(&key2).cloned() {
            for key in equated_keys_with_key(key1.clone(), self) {
                self.tc_func_table.insert(key.clone(), tc_func.clone());
            }
        } else if let Some(tc_func) = self.tc_func_table.get(&key1).cloned() {
            for key in equated_keys_with_key(key2.clone(), self) {
                self.tc_func_table.insert(key.clone(), tc_func.clone());
            }
        }
        /* For structs*/
        else if let Some(tc_strct) = self.tc_strct_table.get(&key2).cloned() {
            for key in equated_keys_with_key(key1.clone(), self) {
                self.tc_strct_table.insert(key, tc_strct.clone());
            }
        } else if let Some(tc_strct) = self.tc_strct_table.get(&key1).cloned() {
            for key in equated_keys_with_key(key2.clone(), self) {
                self.tc_strct_table.insert(key, tc_strct.clone());
            }
        }
        /* for arrays */
        else if let Some(tc_array) = self.tc_array_table.get(&key2).cloned() {
            for key in equated_keys_with_key(key1.clone(), self) {
                self.tc_array_table.insert(key, tc_array.clone());
            }
        } else if let Some(tc_array) = self.tc_array_table.get(&key1).cloned() {
            for key in equated_keys_with_key(key2.clone(), self) {
                self.tc_array_table.insert(key, tc_array.clone());
            }
        }
        /* for optionals */
        else if let Some(tc_opt) = self.tc_optional_table.get(&key2).cloned() {
            for key in equated_keys_with_key(key1.clone(), self) {
                self.tc_optional_table.insert(key, tc_opt.clone());
            }
        } else if let Some(tc_opt) = self.tc_optional_table.get(&key1).cloned() {
            for key in equated_keys_with_key(key2.clone(), self) {
                self.tc_optional_table.insert(key, tc_opt.clone());
            }
        }
    }

    fn concretizes_key(&mut self, key: TcKey, ty: ValueType) {
        let concretizes_lib_key = |key: TcKey, ty: ValueType| {
            key.concretizes_explicit(ty.subst_generic_ty(ValueType::Unspecified))
        };

        let mut concretize_key_with_strct = |strct_name: &str, key: TcKey| {
            if let Some(tc_strct) = self
                .expect_strct_from_usage(strct_name, self.get_item_of(&key).clone())
                .cloned()
            {
                self.tc_strct_table.insert(key, tc_strct);
            } else {
                warn!("Could not find strct with name {}", strct_name);
            }
        };

        if let Some(func_ty) = ty.as_func() {
            let tc_func = TcFunc::from_signature(&*func_ty, self); // Generate func first
            self.equate_keys(key, tc_func.self_key) // Set term equal to func
        } else if let Some(generic_name) = ty.as_generic() {
            debug!(
                "Not concretizising {}({:?}) with generic ty {}",
                self.get_item_of(&key),
                key,
                ty
            );
            // We never concretizes a key to be generic. doesnt make sense
            self.tc_generic_table.insert(key, generic_name.clone()); // No further concretization needed
        } else if let Some((inner_ty, inner_ty_decl)) = ty.as_array() {
            let inner_ty_key =
                self.new_term_key_concretiziesd(inner_ty_decl.clone(), *inner_ty.clone());
            self.tc_array_table.insert(key, inner_ty_key);

            let res = self.checker.impose(concretizes_lib_key(key, ty.clone()));
            self.handle_tc_result(res);
        } else if let Some((inner_ty, inner_ty_decl)) = ty.as_optional() {
            let inner_ty_key =
                self.new_term_key_concretiziesd(inner_ty_decl.clone(), *inner_ty.clone());
            self.tc_optional_table.insert(key, inner_ty_key);

            let res = self.checker.impose(concretizes_lib_key(key, ty.clone()));
            self.handle_tc_result(res);
        } else if let Some(strct_decl) = ty.as_strct() {
            // panic!("When does this ever happen?");
            let strct_decl = Weak::upgrade(strct_decl).unwrap();
            let l_strct_decl = strct_decl
                .try_read_recursive_for(Duration::new(5, 0))
                .expect("Could not acquire lock");
            concretize_key_with_strct(&l_strct_decl.name, key.clone());
            let res = self.checker.impose(concretizes_lib_key(
                key,
                ValueType::StrctName(l_strct_decl.name.clone()),
            ));
            self.handle_tc_result(res);
        } else if let Some(strct_name) = ty.as_strct_name() {
            concretize_key_with_strct(strct_name, key.clone());
            let res = self.checker.impose(concretizes_lib_key(key, ty.clone()));
            self.handle_tc_result(res);
        } else {
            let res = self.checker.impose(concretizes_lib_key(key, ty.clone()));
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

    #[allow(dead_code)]
    pub(crate) fn as_result(self) -> LuResults<TyCheckState> {
        if self.failed() {
            Err(self.collect_all_errors())
        } else {
            Ok(self)
        }
    }

    /// Gets or inserts a key + tc_func for cmd with name `cmd_name`, that could be called with
    /// the `passed_flags`
    fn get_key_of_cmd(&mut self, func_name: &str, passed_flags: &[FlagVariant]) -> Option<TcKey> {
        if let Some(func) = self.scope.find_func(func_name, &passed_flags).cloned() {
            let already_inserted_key = self.tc_var_cmd_table.iter().find_map(|(cmd, key)| {
                if Rc::ptr_eq(&func, cmd) {
                    Some(key)
                } else {
                    None
                }
            });
            if let Some(key) = already_inserted_key {
                Some(key.clone())
            } else {
                // Func is in scope, but doesn't have a tc_key yet ( first time usage of the func )
                debug!(
                    "Found cmd {}, which has no tc_key yet. Inserting new tc_func",
                    func_name
                );
                let tc_func = TcFunc::from_signature(func.signature(), self);
                let tc_func_self_key = tc_func.self_key.clone();
                self.tc_var_cmd_table
                    .push((func.clone(), tc_func.self_key.clone()));
                self.tc_func_table.insert(tc_func.self_key.clone(), tc_func);
                Some(tc_func_self_key)
            }
        } else {
            None
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
                if let Some(callable) = var.val.as_command() {
                    debug!(
                        "First time usage of func {}. Inserting new tc_func.",
                        var_name
                    );
                    let tc_func = TcFunc::from_signature(callable.signature(), self);
                    self.tc_var_table
                        .insert(var.clone(), tc_func.self_key.clone());
                    Some(tc_func.self_key)
                } else if let Some(strct) = var.val.as_strct_decl().cloned() {
                    let strct_name = { strct.read_recursive().name.clone() };
                    debug!(
                        "First time usage of a strct {}. Inserting new tc_strct.",
                        strct_name
                    );
                    let tc_strct = TcStrct::from_strct(&strct, self);
                    self.tc_var_table
                        .insert(var.clone(), tc_strct.self_key.clone());

                    // TODO TcStrct cant concretize itself because of recursion...
                    debug!("Concretizing tc strct to be of strctname ty");
                    self.concretizes_key(
                        tc_strct.self_key.clone(),
                        ValueType::StrctName(strct_name),
                    );

                    Some(tc_strct.self_key)
                } else {
                    warn!("Var is present, but not func: {}. inserting it", var_name);
                    Some(self.insert_var(var))
                }
            }
        } else {
            None
        }
    }

    pub fn expect_key_from_cmd(
        &mut self,
        cmd_name: &str,
        required_flags: &[FlagVariant],
        usage: SourceCodeItem,
    ) -> Option<TcKey> {
        if let Some(cmd_key) = self.get_key_of_cmd(cmd_name, required_flags) {
            Some(cmd_key)
        } else {
            self.push_err(AstErr::CmdNotInScope(usage.clone()).into());
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
            debug!("Expected key {:?} with item {} to be a struct", key, item);
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

    fn get_tc_cmd_from_key(&mut self, key: TcKey) -> Option<TcFunc> {
        self.get_tc_func(&key.clone())
            .cloned()
            .map(|tc_func| tc_func.substitute_generics(self))
    }

    pub fn get_tc_cmd_from_cmd_usage(
        &mut self,
        var_name: &str,
        passed_flags: &[FlagVariant],
    ) -> Option<TcFunc> {
        self.get_key_of_cmd(var_name, passed_flags)
            .map(|key| self.get_tc_cmd_from_key(key))
            .flatten()
    }

    pub fn get_tc_cmd_from_rc_cmd(&mut self, cmd: &Rc<dyn Command>) -> Option<TcFunc> {
        self.get_key_of_cmd(cmd.name(), &cmd.signature().req_flags())
            .map(|key| self.get_tc_cmd_from_key(key))
            .flatten()
    }

    /// Returns the strct behind key if key is a Strct. Records an error otherwise
    /// Therefore the user does not have to handle the None case
    fn expect_tc_cmd_from_key(&mut self, key: TcKey) -> Option<TcFunc> {
        if let Some(cmd) = self.get_tc_cmd_from_key(key) {
            Some(cmd)
        } else {
            let key_item = self.get_item_of(&key).clone();
            self.push_err(TyErr::ItemExpectedToBeFunc(key_item).into());
            None
        }
    }

    /// Some if such a callable is found. None otherwise (and an error will be generated)
    fn expect_tc_cmd_from_cmd_usage(
        &mut self,
        cllbl_name: &str,
        required_flags: &[FlagVariant],
        cllbl_usage: SourceCodeItem,
    ) -> Option<TcFunc> {
        if let Some(func_key) = self.expect_key_from_cmd(&cllbl_name, required_flags, cllbl_usage) {
            self.expect_tc_cmd_from_key(func_key)
        } else {
            None
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

    /// Returns the inner_ty key behind key if key is a optional. Records an error otherwise
    /// Therefore the user does not have to handle the None case
    fn expect_opt_inner_ty_from_key(&mut self, opt_key: TcKey) -> Option<TcKey> {
        let inner_ty_key = self.get_optional_inner_tc(&opt_key.clone()).cloned();

        if inner_ty_key.is_none() {
            let key_item = self.get_item_of(&opt_key).clone();
            self.push_err(TyErr::ItemExpectedToBeOptional(key_item).into());
        }
        inner_ty_key
    }

    /// Get the SourceCodeItem behind the key
    pub(crate) fn get_item_of(&self, key: &TcKey) -> &SourceCodeItem {
        self.tc_expr_table.get(key).unwrap()
    }

    /// Insert var var with ty ty
    fn insert_var(&mut self, var: Variable) -> TcKey {
        self.scope.get_cur_frame_mut().insert_var(var.clone());
        let key = self.new_term_key(var.decl.clone());
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

    fn get_optional_inner_tc(&self, key: &TcKey) -> Option<&TcKey> {
        self.tc_optional_table.get(key)
    }
}

impl PipelineStage for TyCheckState {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        None
    }

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}

#[derive(Debug, Clone, new)]
pub struct TcStrctField {
    pub name: String,
    pub ty: TcKey,
    pub val_ty: ValueType,
    pub field_num: u32,
}
#[derive(Debug, Clone, new)]
pub struct TcStrct {
    /// Key of this strct decl. (used to get SourceCodeItem from tc_expr_table)
    self_key: TcKey,
    /// Always sorted by field name
    field_keys: Vec<TcStrctField>,
}

impl TcStrct {
    pub fn from_strct(strct: &Arc<RwLock<Strct>>, ty_state: &mut TyCheckState) -> Self {
        let tc_strct = {
            let l_strct = strct.read_recursive();
            debug!("Generating TcStrct for Struct: {:?}", strct);
            // TODO when cocnretizing self_key its stack overflow because of recursie func call
            // from_strct -> concretize_key -> expect_strct_from_key -> from_strct
            let self_key = ty_state.new_term_key(l_strct.decl.clone());
            let field_keys = l_strct
                .fields
                .iter()
                .map(|field| TcStrctField {
                    name: field.name.clone(),
                    // This may deadlock if field.ty is same strct as this one
                    ty: ty_state.new_term_key_concretiziesd(field.decl.clone(), field.ty.clone()),
                    field_num: field.field_num,
                    val_ty: field.ty.clone(),
                })
                .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
                .collect();

            Self {
                self_key,
                field_keys,
            }
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
    args_keys: Vec<(ArgSignature, TcKey)>,
    var_arg_key: Option<TcKey>,
    flags_keys: Vec<(FlagSignature, TcKey)>,
}

impl TcFunc {
    pub(crate) fn substitute_generics(self, ty_state: &mut TyCheckState) -> TcFunc {
        debug!(
            "Substituting generics in: {}",
            ty_state.get_item_of(&self.self_key)
        );
        let mut generics_key = HashMap::<String, TcKey>::new();
        let result = self.substitute_generics_rec(&mut generics_key, ty_state);
        debug!(
            "Finished Substituting generics in: {}",
            ty_state.get_item_of(&result.self_key)
        );
        debug!(
            "Inserting with self_key {:?} TcFunc {:?}",
            result.self_key, result
        );
        ty_state
            .tc_func_table
            .insert(result.self_key.clone(), result.clone());
        trace!("{:?}", result);
        result
    }

    fn substitute_generics_rec(
        mut self,
        seen_generics: &mut HashMap<String, TcKey>,
        ty_state: &mut TyCheckState,
    ) -> TcFunc {
        fn subst_generic_key(
            key: TcKey,
            seen_generics: &mut HashMap<String, TcKey>,
            ty_state: &mut TyCheckState,
        ) -> TcKey {
            if let Some(generic_name) = ty_state.get_tc_generic(&key).cloned() {
                let key_item = ty_state.get_item_of(&key).clone();
                // Generic key which needs to be substituted
                let generic_key = ty_state.new_term_key(key_item.clone());
                match seen_generics.entry(generic_name.clone()) {
                    Entry::Occupied(already_inserted_key) => {
                        let already_inserted_generic_item =
                            ty_state.get_item_of(already_inserted_key.get());
                        debug!(
                            "Unifying {} with other generic {}",
                            key_item, already_inserted_generic_item
                        );
                        ty_state.equate_keys(generic_key, already_inserted_key.get().clone());
                    }
                    Entry::Vacant(v) => {
                        debug!(
                            "Found generic ty: {} for first time and generated key {:?}",
                            key_item, generic_key
                        );
                        v.insert(generic_key);
                    }
                };
                generic_key
            } else if let Some(tc_func) = ty_state.get_tc_func(&key).cloned() {
                trace!(
                    "Substitute Generics: Found inner func_ty({}). Recursing into that",
                    ty_state.get_item_of(&tc_func.self_key)
                );
                let key = tc_func.self_key.clone();
                let new_tc_func = tc_func.substitute_generics_rec(seen_generics, ty_state);
                //Update changed tc_func
                ty_state
                    .tc_func_table
                    .insert(new_tc_func.self_key, new_tc_func);
                key
            } else if let Some(inner_arr_key) = ty_state.get_arr_inner_tc(&key).cloned() {
                trace!("Substitute Generics: Found inner array_ty. Recursing into that");
                let new_inner_arr_key = subst_generic_key(inner_arr_key, seen_generics, ty_state);
                ty_state.tc_array_table.insert(key, new_inner_arr_key); // TODO bit of direct access here...
                key
            } else if let Some(inner_opt_key) = ty_state.get_optional_inner_tc(&key).cloned() {
                trace!("Substitute Generics: Found inner optional. Recursing into that");
                let new_inner_opt_key = subst_generic_key(inner_opt_key, seen_generics, ty_state);
                ty_state.tc_optional_table.insert(key, new_inner_opt_key); // TODO bit of direct access here...
                key
            } else {
                trace!(
                    "Found non generic normal key {}. Not substituting",
                    ty_state.get_item_of(&key)
                );
                key
            }
        }

        self.in_key = subst_generic_key(self.in_key, seen_generics, ty_state);
        self.ret_key = subst_generic_key(self.ret_key, seen_generics, ty_state);
        if let Some(var_arg_key) = &mut self.var_arg_key {
            *var_arg_key = subst_generic_key(*var_arg_key, seen_generics, ty_state);
        }
        for (_, arg_key) in self.args_keys.iter_mut() {
            *arg_key = subst_generic_key(*arg_key, seen_generics, ty_state)
        }
        // TODO handle flags

        self
    }

    /// generate a TcFunc from func. Each arg / flag / in / ret type of the func
    /// will be inserted as a seperate pseudo variable
    pub fn from_signature(sign: &Signature, ty_state: &mut TyCheckState) -> Self {
        debug!("Generating TcFunc for Signature: {:?}", sign);
        // TODO concretizising the self_key with ValueType::Func will result in endless recursion
        let self_key = ty_state.new_term_key(sign.decl.clone()); 
        // let self_key = ty_state
        //     .new_term_key_concretiziesd(sign.decl.clone(), ValueType::Func(Box::new(sign.clone())));

        let in_key =
            ty_state.new_term_key_concretiziesd(sign.in_arg.decl.clone(), sign.in_arg.ty.clone());
        debug!("generated new in key, now gen ret key");

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
                let arg_key =
                    ty_state.new_term_key_concretiziesd(arg_sign.decl.clone(), arg_sign.ty.clone());
                (arg_sign.clone(), arg_key)
            })
            .collect();

        let flags_keys = sign
            .flags
            .iter()
            .map(|flag| {
                let flag_key =
                    ty_state.new_term_key_concretiziesd(flag.decl.clone(), flag.ty.clone());
                (flag.clone(), flag_key)
            })
            .collect();

        let ty_func = Self {
            self_key,
            in_key,
            ret_key,
            args_keys,
            var_arg_key,
            flags_keys,
        };

        ty_state
            .tc_func_table
            .insert(ty_func.self_key.clone(), ty_func.clone());
        debug!(
            "Generated TcFunc with keys: in: {:?}, ret: {:?}",
            ty_func.in_key, ty_func.ret_key
        );

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
        let self_args_key_iter = self
            .args_keys
            .iter()
            .map(|(_, key)| key)
            .chain(self.var_arg_key.as_ref());
        let other_args_key_iter = other
            .args_keys
            .iter()
            .map(|(_, key)| key)
            .chain(other.var_arg_key.as_ref());
        let args_constr = itertools::zip(self_args_key_iter, other_args_key_iter)
            .map(|(self_arg_key, other_arg_key)| self_arg_key.equate_with(*other_arg_key))
            .chain(in_ret_constr);

        for constr in args_constr {
            let res = ty_state.checker.impose(constr);
            ty_state.handle_tc_result(res);
        }
    }
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum TypeCheckArg {
    Arg(VisitArg),
    CmdStmt { in_piped_arg_key: TcKey },
}

pub trait TypeCheck: Display {
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
        debug!("Typechecking: {}, with {} args", self, args.len());
        let result = self.do_typecheck(args, ty_state);
        debug!(
            "Result of Typechecking: {}: {:?}",
            self,
            // TODO better debug stmt
            result,
        );
        result
    }
}
