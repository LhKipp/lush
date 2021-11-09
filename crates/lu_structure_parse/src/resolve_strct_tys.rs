use std::{collections::HashMap, rc::Rc, sync::Arc};

use log::{debug, warn};
use lu_error::{util::Outcome, AstErr, LuErr, SourceCodeItem};
use lu_interpreter_structs::*;
use parking_lot::RwLock;

pub(crate) fn resolve_strct_types(
    mut modules: Vec<ScopeFrame<Variable>>,
) -> Outcome<Vec<ScopeFrame<Variable>>> {
    debug!("Resolving ValueType::StrctName");
    let mut errs = vec![];

    let all_struct_decls: HashMap<_, _> = modules
        .iter()
        .map(|module| module.elems.iter())
        .flatten()
        .filter_map(|(_, var)| var.val.as_strct_decl().cloned())
        .map(|strct| {
            let name = strct.read().name.clone();
            (name, strct)
        })
        .collect();

    for frame in modules.iter_mut() {
        for (_, var) in frame.elems.iter_mut() {
            subst_value_tys(&mut var.val, &all_struct_decls, &mut errs);
        }
    }
    Outcome::new(modules, errs)
}

fn subst_value_tys(
    value: &mut Value,
    all_struct_decls: &HashMap<String, Arc<RwLock<Strct>>>,
    errs: &mut Vec<LuErr>,
) {
    if let Value::StrctDecl(strct) = value {
        let mut w_strct = strct.write();
        for field in &mut w_strct.fields {
            subst_strct_name(&mut field.ty, &all_struct_decls, &field.decl)
                .map(|err| errs.push(err));
        }
    } else if let Value::Command(func) = value {
        let l_func = Rc::get_mut(func).expect("No references yet");
        if let Some(func) = l_func.as_function_mut() {
            let sign = &mut func.signature;
            sign.args.iter_mut().for_each(|arg| {
                subst_strct_name(&mut arg.ty, &all_struct_decls, &arg.decl)
                    .map(|err| errs.push(err));
            });
            sign.flags.iter_mut().for_each(|flag| {
                subst_strct_name(&mut flag.ty, &all_struct_decls, &flag.decl)
                    .map(|err| errs.push(err));
            });
            subst_strct_name(&mut sign.in_arg.ty, &all_struct_decls, &sign.in_arg.decl)
                .map(|err| errs.push(err));
            subst_strct_name(&mut sign.ret_arg.ty, &all_struct_decls, &sign.ret_arg.decl)
                .map(|err| errs.push(err));
            if let Some(var_arg) = &mut sign.var_arg {
                subst_strct_name(&mut var_arg.ty, &all_struct_decls, &var_arg.decl)
                    .map(|err| errs.push(err));
            }
        }
    } else {
        unreachable!("Only commands and strcts are yet sourced")
    }
}
fn subst_strct_name(
    ty: &mut ValueType,
    all_struct_decls: &HashMap<String, Arc<RwLock<Strct>>>,
    decl: &SourceCodeItem,
) -> Option<LuErr> {
    if let ValueType::StrctName(strct_name) = ty {
        debug!("Found ValueType::StrctName({}) to substitute", strct_name);
        if let Some(strct_decl) = all_struct_decls.get(strct_name) {
            *ty = ValueType::Strct(Arc::downgrade(strct_decl));
            None
        } else {
            Some(AstErr::StrctNotInScope(decl.clone()).into())
        }
    } else if let ValueType::Array { inner_ty, .. } = ty {
        subst_strct_name(inner_ty, all_struct_decls, decl)
    } else {
        warn!("Not substituting ValueType::StrctName in inner fn tys");
        None
    }
}
