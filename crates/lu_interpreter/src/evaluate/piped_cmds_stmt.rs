use lu_error::LuResult;
use lu_syntax::ast::PipedCmdsStmtNode;
use lu_value::Value;

use crate::{EvalArg, Evaluable, Evaluator, Variable};

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let mut previous_val = Value::Nil;
        for cmd in self.cmds() {
            state
                .scope
                .lock()
                .cur_mut_frame()
                .insert("in".to_string(), Variable::new_in(previous_val));
            previous_val = cmd.evaluate(state)?
        }

        Ok(previous_val)
    }
}
