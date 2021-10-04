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

        let fn_frame = ScopeFrameTag::FnFrame(fn_name.clone());
        ty_state.scope.push_frame(fn_frame.clone());

        let var_key_to_insert = {
            let own_tc_func = ty_state
                .expect_callable_from_var(&fn_name, self.decl_item())
                .expect("Always works");
            let own_signature = ty_state
                .scope
                .find_func(&fn_name)
                .expect("FnNode will be sourced")
                .signature();
            let mut var_ty_to_insert = Vec::new();

            for (arg, key) in own_signature.args.iter().zip(own_tc_func.args_keys) {
                var_ty_to_insert.push((arg.to_var(), key));
            }
            var_ty_to_insert.push((own_signature.in_arg.to_var(), own_tc_func.in_key));
            if let Some(var_arg) = &own_signature.var_arg {
                var_ty_to_insert.push((var_arg.to_var(), own_tc_func.var_arg_key.unwrap()));
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
