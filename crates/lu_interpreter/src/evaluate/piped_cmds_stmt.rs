use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::PipedCmdsStmtNode;

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let mut prev_val = None;
        for cmd in self.cmds() {
            if let Some((prev_val, prev_val_decl)) = prev_val {
                scope
                    .lock()
                    .get_cur_frame_mut()
                    .insert_var(Variable::new_in(prev_val, prev_val_decl));
            }
            prev_val = Some((cmd.evaluate(scope)?, cmd.clone().into()))
        }

        Ok(prev_val
            .expect("PipedCmdsStmtNode is always at least 1 cmd")
            .0)
    }
}
