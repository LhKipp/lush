use log::warn;
use lu_syntax::{ast::RetStmtNode, AstToken};
use rusttyc::TcKey;

use crate::{TyCheckState, ValueType};

use super::TypeCheck;

impl TypeCheck for RetStmtNode {
    fn do_typecheck(
        &self,
        _: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        let self_item = self.ret_kw().to_item();

        let cur_func_name = ty_state
            .scope
            .get_cur_func()
            .map(|callable| callable.name().to_string());

        if let Some(cur_func_name) = cur_func_name {
            if let Some(tc_func) =
                ty_state.expect_callable_from_var(&cur_func_name, self_item.clone())
            {
                let ret_stmt_key =
                    ty_state.new_term_key_equated(self_item, tc_func.ret_key.clone());
                if let Some(ret_value) = self.returned_val() {
                    // Returned value must be compatible with ret decl
                    let ret_value_key = ret_value.typecheck(ty_state);
                    ty_state
                        .equate_keys(ret_stmt_key, ret_value_key.expect("value always returns"));
                } else {
                    // Returning nothing. Ret has to be void
                    ty_state.concretizes_key(ret_stmt_key, ValueType::Void);
                }
            }
        } else {
            warn!("RetStmt outside of a fn. Shouldn't happen");
        }

        // A ret stmt doesn't return a value
        None
    }
}
