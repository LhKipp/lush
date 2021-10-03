use lu_syntax::ast::PipedCmdsStmtNode;
use lu_syntax_elements::constants::IN_ARG_NAME;

use crate::{EvalArg, EvalResult, Evaluable, Evaluator, Variable};

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        let mut prev_val = None;
        for cmd in self.cmds() {
            if let Some((prev_val, prev_val_decl)) = prev_val {
                state.scope.lock().cur_mut_frame().insert(
                    IN_ARG_NAME.to_string(),
                    Variable::new_in(prev_val, prev_val_decl),
                );
            }
            prev_val = Some((cmd.evaluate(state)?, cmd.clone().into()))
        }

        Ok(prev_val
            .expect("PipedCmdsStmtNode is always at least 1 cmd")
            .0)
    }
}
