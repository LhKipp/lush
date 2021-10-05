use lu_error::TyErr;
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
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
            // TODO get ty first and then insert var once
            let let_stmt_key = ty_state.insert_var(var.clone());

            // unify key with decl
            if let Some(decl_ty) = self.decl_ty() {
                let (ty, err) = ValueType::from_node_or_err_ty(&decl_ty.into_type());
                ty_state.record_option(err);
                ty_state.concretizes_key(let_stmt_key, ty);
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
                    ty_state.push_err(TyErr::TermDoesNotReturnType(rhs.unwrap().to_item()).into())
                }
            };
        } else {
            // Incomplete let stmt in parsing. This is okay
        }

        // LetStmt does not have a return value
        None
    }
}
