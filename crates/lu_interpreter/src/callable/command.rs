use std::fmt::Debug;
use std::rc::Rc;

use crate::{EvalArg, Interpreter, Scope, Variable};

use log::debug;
use lu_error::LuResult;
use lu_value::{Value, NIL_VAL};

// pub struct CommandArgs {
//     /// The name by which the command has been called
//     pub cmd_call_name: Vec<String>,
//     /// The value from stdin
//     pub input: Value,
//     /// The arguments of the command
// }

// impl CommandArgs {
//     pub fn new() -> Self {
//         CommandArgs {
//             cmd_call_name: Vec::new(),
//             input: Value::Nil,
//         }
//     }
// }

pub const IN_VAR_NAME: &str = "in";
pub const ARGS_VAR_NAME: &str = "args";

pub trait Command: CommandClone + Debug {
    fn name(&self) -> &str;

    /// Returns $args
    fn expect_args<'a>(&self, scope: &'a Scope<Variable>) -> &'a Rc<Vec<Value>> {
        match &scope.find_var(ARGS_VAR_NAME).expect("Always present").val {
            Value::Array(v) => &v,
            _ => unreachable!("Args are always an array"),
        }
    }

    /// Returns $in
    fn expect_in<'a>(&self, scope: &'a Scope<Variable>) -> &'a Value {
        &scope
            .find_var(IN_VAR_NAME)
            .map(|var| &var.val)
            .unwrap_or(&NIL_VAL)
    }

    fn do_run(&self, args: &[EvalArg], state: &mut Interpreter) -> LuResult<Value>;

    fn run(&self, state: &mut Interpreter) -> LuResult<Value> {
        self.run_with_args(&[], state)
    }

    fn run_with_args(&self, args: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        {
            let l_scope = state.scope.lock();
            debug!(
                "Running command {:?} with args ({:?})\n$in: {:?}, $args {:?}",
                self.name(),
                args,
                self.expect_in(&l_scope),
                self.expect_args(&l_scope)
            )
        }
        let result = self.do_run(args, state);
        {
            debug!("Result of running command {}: {:?}", self.name(), result);
        }

        result
    }
}

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}
