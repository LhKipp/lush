#![allow(dead_code)]
#![allow(unused_imports)]
mod command_storage;

use lu_error::{LuErr, LuResult, ParseErr};
use lu_text_util::SourceCode;
use syntax::{ast::SourceFileNode, Parse};
use value::Value;

use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::{Evaluable, Scope};
pub use command_storage::CommandStorage;

/// The interpreter holds data, getting transformed while interpreting the ast.
/// The interpreter struct is merely here for having a nice frontend to the interpreter crate
pub struct Interpreter {
    pub cmds: Arc<Mutex<CommandStorage>>,
    pub scope: Arc<Mutex<Scope>>,
}

impl Interpreter {
    pub fn new(cmds: CommandStorage) -> Self {
        Interpreter {
            cmds: Arc::new(Mutex::new(cmds)),
            scope: Arc::new(Mutex::new(Scope::new())),
        }
    }

    pub fn evaluate(&mut self, code: SourceCode) -> LuResult<Value> {
        let parse_result = Parse::source_file(&code.to_string()?);
        // We don't allow evaluation if errors happend.
        let source_file = parse_result.ok::<SourceFileNode>()?;
        source_file.evaluate(self)
    }

    pub fn evaluate_node(&mut self, node: &dyn Evaluable) -> LuResult<Value> {
        node.evaluate(self)
    }
}
