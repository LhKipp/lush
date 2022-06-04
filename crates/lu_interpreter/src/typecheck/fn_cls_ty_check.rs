use lu_interpreter_structs::{Signature, ValueType};

use crate::{TcFunc, TyCheckState};

pub(crate) fn insert_arguments_into_scope(
    tc_func: TcFunc,
    sign: &Signature,
    ty_state: &mut TyCheckState,
) {
    let var_key_to_insert = {
        let mut var_ty_to_insert = Vec::new();

        for (arg, key) in tc_func.args_keys {
            if arg.is_opt {
                // optional arg is inserted as optional<ty>
                let key = ty_state.new_term_key_concretiziesd(
                    arg.decl.clone(),
                    ValueType::Optional {
                        inner_ty: Box::new(arg.ty.clone()),
                        inner_ty_decl: arg.decl.clone(),
                    },
                );
                var_ty_to_insert.push((arg.to_var(), key))
            } else {
                var_ty_to_insert.push((arg.to_var(), key));
            }
        }

        var_ty_to_insert.push((sign.in_arg.to_var(), tc_func.in_key));
        if let Some(var_arg) = &sign.var_arg {
            var_ty_to_insert.push((var_arg.to_var(), tc_func.var_arg_key.unwrap()));
        }
        for (flag, key) in tc_func.flags_keys {
            if flag.ty.is_bool() || flag.is_required() {
                var_ty_to_insert.push((flag.to_var(), key))
            } else {
                // optional flag and ty is not bool, inserted flag is optional then
                let key = ty_state.new_term_key_concretiziesd(
                    flag.decl.clone(),
                    ValueType::Optional {
                        inner_ty: Box::new(flag.ty.clone()),
                        inner_ty_decl: flag.decl.clone(), // TODO fixup ty decl
                    },
                );
                var_ty_to_insert.push((flag.to_var(), key))
            }
        }
        var_ty_to_insert
    };

    for (var, key) in var_key_to_insert {
        let var_key = ty_state.insert_var(var);
        ty_state.equate_keys(var_key, key);
    }
}
