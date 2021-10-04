use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::PipedCmdsStmtNode;
use lu_syntax_elements::constants::IN_ARG_NAME;

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let mut prev_val = None;
        for cmd in self.cmds() {
            if let Some((prev_val, prev_val_decl)) = prev_val {
                scope.lock().cur_mut_frame().insert(
                    IN_ARG_NAME.to_string(),
                    Variable::new_in(prev_val, prev_val_decl),
                );
            }
            prev_val = Some((cmd.evaluate(scope)?, cmd.clone().into()))
        }

        Ok(prev_val
            .expect("PipedCmdsStmtNode is always at least 1 cmd")
            .0)
    }
}
