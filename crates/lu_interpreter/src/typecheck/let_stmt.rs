use lu_error::TyErr;
use lu_pipeline_stage::ErrorContainer;
use lu_syntax::{ast::LetStmtNode, AstElement};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable};

impl TypeCheck for LetStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        if let Some(var_name) = self.var_name() {
            let var = Variable::new(var_name, Value::Nil, VarDeclNode::LetStmt(self.clone()));
            let let_stmt_key = ty_state.new_term_key(self.item_till_value());

            ty_state.insert_var(var.clone(), let_stmt_key);

            // unify key with decl
            if let Some(decl_ty) = self.decl_ty() {
                let ty = ValueType::from_node(&decl_ty.into_type(), &ty_state.scope);
                let ty = ty_state.ok_or_record(ty).unwrap_or(ValueType::Error);
                let tc_res = ty_state
                    .checker
                    .impose(let_stmt_key.concretizes_explicit(ty));
                ty_state.handle_tc_result(tc_res);
            }

            // Combine key with rhs
            let rhs = self.value();
            match rhs.typecheck(ty_state) {
                Some(key) => {
                    ty_state.equate_keys(let_stmt_key, key);
                }
                None => {
                    // Example of this path would be: let x = { let y = 1 }
                    // The block does not return a type
                    assert!(rhs.is_some(), "Option<T> always returns something for none");
                    ty_state
                        .errors
                        .push(TyErr::TermDoesNotReturnType(rhs.unwrap().to_item()).into())
                }
            };
        } else {
            // Incomplete let stmt in parsing. This is okay
        }

        // LetStmt does not have a return value
        None
    }
}
