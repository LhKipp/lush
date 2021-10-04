use std::fmt::Debug;
use std::rc::Rc;

use crate::{EvalArg, Evaluator, Scope, Signature, VarDeclNode, Variable};

use log::debug;
use lu_error::{LuResult, SourceCodeItem};
use lu_value::Value;

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
/// Default arg names are arg0 arg1 ...
pub const ARG_VAR_NAME: &str = "arg";

pub trait Command: CommandClone + Debug {
    fn name(&self) -> &str;

    fn signature(&self) -> &Signature;

    /// Returns SourceCodeItem into the signature/declaration of the command
    fn signature_item(&self) -> SourceCodeItem;

    /// Returns $args
    fn expect_args<'a>(&self, scope: &'a Scope<Variable>) -> &'a Rc<Vec<Value>> {
        match &scope.find_var(ARGS_VAR_NAME).expect("Always present").val {
            Value::Array(v) => &v,
            _ => unreachable!("Args are always an array"),
        }
    }

    /// Returns $in
    fn expect_in<'a>(&self, scope: &'a Scope<Variable>) -> &'a Value {
        self.expect_arg(scope, IN_VAR_NAME)
    }

    /// Returns $<arg_name>
    fn expect_arg<'a>(&self, scope: &'a Scope<Variable>, arg_name: &str) -> &'a Value {
        &scope
            .find_var(arg_name)
            .map(|var| &var.val)
            .expect("Variable always present")
    }

    /// Returns $<arg_name>
    fn expect_mut_arg<'a>(&self, scope: &'a mut Scope<Variable>, arg_name: &str) -> &'a mut Value {
        &mut scope
            .find_var_mut(arg_name)
            .expect("Variable always present")
            .val
    }

    // /// Returns $<arg_name>
    // fn expect_overwrite_var<'a>(
    //     &self,
    //     scope: &'a mut Scope<Variable>,
    //     var_name: &str,
    //     new_val: Value,
    // ) {
    //     assert!(&scope.overwrite_var_value(var_name, new_val))
    // }

    fn do_run(&self, args: &[EvalArg], state: &mut Evaluator) -> LuResult<Value>;

    fn run(&self, state: &mut Evaluator) -> LuResult<Value> {
        self.run_with_args(&[], state)
    }

    fn run_with_args(&self, args: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
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

    fn boxed(self) -> Box<dyn Command>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
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

impl Into<Variable> for Box<dyn Command> {
    fn into(self) -> Variable {
        let name = self.name().to_string();
        let decl = self.signature().decl.clone();
        let value = Value::new_func(self);
        Variable::new(name, value, VarDeclNode::CatchAll(decl))
    }
}
