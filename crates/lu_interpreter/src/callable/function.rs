#![allow(dead_code)]
use crate::Evaluator;
use crate::{Command, Evaluable, Function, Signature};
use lu_error::SourceCodeItem;
use lu_value::Value;

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run(
        &self,
        _: &[crate::EvalArg],
        state: &mut Evaluator,
    ) -> lu_error::LuResult<lu_value::Value> {
        // TODO typecheck and put vars into scope
        if let Some(block) = self.fn_node.block_stmt() {
            block.evaluate(state)
        } else {
            Ok(Value::Nil)
        }
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn signature_item(&self) -> SourceCodeItem {
        self.fn_node.decl_item()
    }
}
