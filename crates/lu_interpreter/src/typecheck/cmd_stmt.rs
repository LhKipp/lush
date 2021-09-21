#![allow(unused_imports)]
#![allow(unused_variables)]
use log::debug;
use lu_error::TyErr;
use lu_pipeline_stage::ErrorContainer;
use lu_syntax::{
    ast::{CmdStmtNode, LetStmtNode},
    AstElement, AstNode,
};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{Callable, TypeCheck, TypeCheckArg, ValueType, VarDeclNode, Variable};

impl TypeCheck for CmdStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TypeChecker,
    ) -> Option<TcKey> {
        debug!("Scope: {:?}", ty_state.scope);
        debug!("Cur Scope Frame: {:?}", ty_state.scope.cur_frame());
        let possibl_longest_name = self.possible_longest_cmd_call_name();
        // Finding result type here
        if let Some(var) = ty_state
            .scope
            .find_var_with_longest_match(&possibl_longest_name)
            .map(|(_, var)| var)
            .cloned()
        {
            let ret_key = ty_state
                .tc_func_table
                .get(&var)
                .map(|func| func.ret_ty)
                .unwrap_or_else(|| {
                    // We have found such a var, but its not a function
                    // This error should be catched more elaborated in special check for this
                    debug!(
                        "Expected {} to be a function, but isn't present in tc_func_table",
                        var.name
                    );
                    ty_state.get_tc_error_key()
                });
            Some(ty_state.new_term_key_equated(self.into_item(), ret_key))
        } else {
            // External cmds return string
            Some(ty_state.new_term_key_concretiziesd(self.into_item(), ValueType::String))
        }
    }
}

#[cfg(test)]
mod test {
    use lu_error::LuResults;
    use lu_test_support::{init_logger, make_test_interpreter};

    use lu_interpreter::ValueType;

    use {conformance, serde_json};

    #[conformance::tests(exact, serde=serde_json, file="test_data/typecheck/cmd_stmt/cmd_simple.json_test")]
    fn general_interpreter_tests(s: &str) -> LuResults<Option<ValueType>> {
        init_logger();
        let mut itprtr = make_test_interpreter();

        itprtr
            .ty_check(s.to_string().into())
            .map(|ty_checker| ty_checker.result)
    }
}
