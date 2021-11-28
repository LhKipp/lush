use lu_interpreter_structs::Value;
use lu_pipeline_stage::ErrorContainer;
use lu_syntax::ast::LetStmtNode;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg, ValueType, Variable};

impl TypeCheck for LetStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        if let Some(var_name) = self.var_name() {
            let var = Variable::new(var_name, Value::Nil, self.item_till_assign());
            // TODO get ty first and then insert var once
            let let_stmt_key = ty_state.insert_var(var.clone());

            // unify key with decl
            if let Some(decl_ty) = self.decl_ty() {
                let ty_out =
                    ValueType::from_node_or_err_resolve_strct_name(&decl_ty, &ty_state.scope);
                let ty = ty_state.ok_and_record(ty_out);
                ty_state.concretizes_key(let_stmt_key, ty);
            }

            // Combine key with rhs
            if let Some(rhs_val) = self.value() {
                let rhs_key = rhs_val
                    .typecheck(ty_state)
                    .expect("Rhs val always returns ty");
                ty_state.equate_keys(let_stmt_key, rhs_key);
            }
        } else {
            // Incomplete let stmt in parsing. This is okay
        }

        // LetStmt does not have a return value
        None
    }
}
