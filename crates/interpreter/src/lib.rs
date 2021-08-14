mod evaluate;
mod evaluation_error;
mod interpreter;

use value::Value;

pub use crate::evaluate::Evaluable;
pub use crate::interpreter::Interpreter;

pub type ShellError = String;

pub struct CommandArgs {
    /// The value which comes from stdin
    pub input: Value,
}

pub trait Command {
    fn name(&self) -> &str;
    fn run(&self, args: CommandArgs, state: &mut Interpreter) -> Result<Value, ShellError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
