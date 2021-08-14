#![allow(dead_code)]
use crate::{Command, Evaluable};

/// The interpreter holds data, getting transformed while interpreting the ast.
/// The interpreter struct is merely here for having a nice frontend to the interpreter crate
pub struct Interpreter {
    cmds: Vec<Box<dyn Command>>,
}

impl Interpreter {
    pub fn evaluate(&mut self, node: &dyn Evaluable) {
        node.evaluate(self);
    }
}
