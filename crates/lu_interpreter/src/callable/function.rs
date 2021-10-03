#![allow(dead_code)]
use crate::{Command, Function, Signature};
use crate::{Evaluable, Evaluator};
use lu_error::{LuResult, SourceCodeItem};
use lu_value::Value;

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run(&self, _: &[crate::EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let eval_result = self.fn_node.evaluate(state);
        Evaluator::eval_result_to_lu_result(eval_result)
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn signature_item(&self) -> SourceCodeItem {
        self.fn_node.decl_item()
    }
}
