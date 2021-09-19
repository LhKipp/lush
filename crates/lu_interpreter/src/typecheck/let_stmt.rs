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
        ty_state: &mut crate::TypeChecker,
    ) -> Option<TcKey> {
        if let Some(var_name) = self.var_name() {
            let var = Variable::new(
                var_name,
                Value::Nil,
                Some(VarDeclNode::LetStmt(self.clone())),
            );
            let let_stmt_key = ty_state.new_term_key(self.item_till_value());

            ty_state.scope.cur_mut_frame().insert_var(var.clone());
            ty_state.tc_table.insert(var, let_stmt_key);

            // unify key with decl
            if let Some(decl_ty) = self.decl_ty() {
                let ty = ValueType::from_node(&decl_ty.into_type());
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
                    let res = ty_state.checker.impose(let_stmt_key.equate_with(key));
                    ty_state.handle_tc_result(res);
                }
                None => {
                    // Example of this path would be: let x = { let y = 1 }
                    // The block does not return a type
                    assert!(rhs.is_some(), "Option<T> always returns something for none");
                    ty_state
                        .errors
                        .push(TyErr::TermDoesNotReturnType(rhs.unwrap().into_item()).into())
                }
            };
        } else {
            // Incomplete let stmt in parsing. This is okay
        }

        // LetStmt does not have a return value
        None
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResults;
    use lu_test_support::{init_logger, make_test_interpreter};
    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/typecheck/let_stmt/wrong_decl.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResults<()> {
        init_logger();
        let mut itprtr = make_test_interpreter();

        itprtr.ty_check(s.to_string().into()).map(|_| ())
    }
}
