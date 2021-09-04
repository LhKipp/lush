#![allow(dead_code)]
use crate::scope::ScopeFrameId;
use crate::{Command, Evaluable, Variable};
use lu_syntax::ast::FnStmtNode;
use lu_value::Value;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    // TODO check whether signature here, or ptr to type checkers resolved t
    // pub signature: Signature,
    pub fn_node: FnStmtNode,
    pub parent_frame_id: ScopeFrameId,
    // For closures only
    pub captured_vars: Vec<Variable>,
}

impl Function {
    pub fn new(
        name: String,
        // signature: Signature,
        fn_node: FnStmtNode,
        parent_frame_id: ScopeFrameId,
    ) -> Self {
        Self {
            name,
            // signature,
            parent_frame_id,
            fn_node,
            captured_vars: Vec::new(),
        }
    }
}

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run(
        &self,
        _: &[crate::EvalArg],
        state: &mut crate::Interpreter,
    ) -> lu_error::LuResult<lu_value::Value> {
        // TODO typecheck and put vars into scope
        if let Some(block) = self.fn_node.block_stmt() {
            block.evaluate(state)
        } else {
            Ok(Value::Nil)
        }
    }
}
