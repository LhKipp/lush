use lu_interpreter_structs::{FlagVariant, ValueType};
use lu_syntax::ast::FnStmtNode;
use rusttyc::TcKey;

use crate::{ScopeFrameTag, TyCheckState};

use super::TypeCheck;

impl TypeCheck for FnStmtNode {
    fn do_typecheck(
        &self,
        _: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        let fn_name = if let Some(name) = self.name() {
            name
        } else {
            return None;
        };
        let required_flags = self
            .signature()
            .map(|sign| {
                sign.flags()
                    .filter(|flag_node| flag_node.is_required())
                    .map(|flag_node| FlagVariant::from_sign_node(&flag_node))
                    .collect()
            })
            .unwrap_or(vec![]);

        let fn_frame = ScopeFrameTag::TyCFnFrame(fn_name.clone(), required_flags.clone());
        ty_state.scope.push_frame(fn_frame.clone());

        let var_key_to_insert = {
            let own_tc_func = ty_state
                .expect_tc_cmd_from_cmd_usage(&fn_name, &required_flags, self.decl_item())
                .expect("Always works");
            let own_signature = ty_state
                .scope
                .find_func(&fn_name, &required_flags)
                .expect("FnNode will be sourced")
                .signature()
                // have to clone for new_term_key_concretiziesd :[
                .clone();
            let mut var_ty_to_insert = Vec::new();

            for (arg, key) in own_signature.args.iter().zip(own_tc_func.args_keys) {
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
            var_ty_to_insert.push((own_signature.in_arg.to_var(), own_tc_func.in_key));
            if let Some(var_arg) = &own_signature.var_arg {
                var_ty_to_insert.push((var_arg.to_var(), own_tc_func.var_arg_key.unwrap()));
            }
            for (flag, key) in own_tc_func.flags_keys {
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

        if let Some(fn_block) = self.block_stmt() {
            fn_block.typecheck(ty_state);
        }

        ty_state.scope.pop_frame(&fn_frame);

        // A fn stmt doesn't return a value
        None
    }
}
