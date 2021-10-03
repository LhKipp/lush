use lu_syntax::ast::LetStmtNode;
use lu_value::Value;

use crate::{variable::VarDeclNode, EvalArg, EvalResult, Evaluable, Evaluator, Variable};

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> EvalResult {
        let var_name = self.var_name().unwrap();
        let val = self.value().unwrap();

        let val = val.evaluate(state)?;

        state.scope.lock().cur_mut_frame().insert(
            var_name.to_string(),
            Variable::new(var_name, val, VarDeclNode::LetStmt(self.clone())),
        );

        Ok(Value::Nil)
    }
}
