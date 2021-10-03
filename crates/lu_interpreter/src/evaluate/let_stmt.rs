use lu_syntax::ast::LetStmtNode;
use lu_value::Value;

use crate::{variable::VarDeclNode, EvalArg, Evaluable, Evaluator, RetValOrErr, Variable};

impl Evaluable for LetStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], state: &mut Evaluator) -> Result<Value, RetValOrErr> {
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
