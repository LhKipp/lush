#![allow(dead_code)]
#![allow(unused_imports)]

use lu_error::{LuErr, LuResult, ParseErr};
use lu_syntax::{
    ast::{HasRule, SourceFileNode},
    AstNode, Parse,
};
use lu_text_util::SourceCode;
use lu_value::Value;

use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::{Evaluable, Scope};

/// The interpreter holds data, getting transformed while interpreting the ast.
/// The interpreter struct is merely here for having a nice frontend to the interpreter crate
pub struct Interpreter {
    pub scope: Arc<Mutex<Scope>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            scope: Arc::new(Mutex::new(Scope::new())),
        }
    }

    pub fn evaluate(&mut self, code: SourceCode) -> LuResult<Value> {
        self.evaluate_as::<SourceFileNode>(code)
    }

    /// Evaluate code as T
    pub fn evaluate_as<T: Evaluable + HasRule + AstNode>(
        &mut self,
        code: SourceCode,
    ) -> LuResult<Value> {
        let parse_result = Parse::rule(&code.to_string()?, &*T::get_belonging_rule());
        // We don't allow evaluation if errors happend.
        let source_file = parse_result.ok::<T>()?;
        source_file.evaluate(self)
    }

    pub fn evaluate_node(&mut self, node: &dyn Evaluable) -> LuResult<Value> {
        node.evaluate(self)
    }
}
