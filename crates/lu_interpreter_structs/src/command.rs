use std::rc::Rc;
use std::{fmt::Debug, sync::Arc};

use crate::{FlagVariant, Function, ModPath, Scope, Signature, Value, VarDeclNode, Variable};

use log::debug;
use lu_error::{LuResult, SourceCodeItem};
use parking_lot::Mutex;

pub const IN_VAR_NAME: &str = "in";
pub const ARGS_VAR_NAME: &str = "args";

pub trait Command: CommandClone + Debug {
    fn name(&self) -> &str;

    fn signature(&self) -> &Signature;

    /// Returns SourceCodeItem into the signature/declaration of the command
    fn signature_item(&self) -> SourceCodeItem;

    /// Returns the module, in which the command is contained / declared
    /// TODO this should be always Some (even for print, ...)
    fn parent_module(&self) -> Option<&ModPath>;

    /// Only overwritten by Function
    fn as_function(&self) -> Option<&Function> {
        None
    }

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

    fn do_run_cmd(&self, scope: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<Value>;

    fn run_cmd(&self, scope: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<Value> {
        debug!("Running command {}", self.name());
        let result = self.do_run_cmd(scope);
        debug!("Result of running command {}: {:?}", self.name(), result);

        result
    }

    fn boxed(self) -> Box<dyn Command>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn rced(self) -> Rc<dyn Command>
    where
        Self: Sized + 'static,
    {
        Rc::new(self)
    }

    /// A command is called if the cmd.name() and at least all required flags are passed
    fn is_called_by(&self, called_cmd_name: &str, passed_flags: &[FlagVariant]) -> bool {
        self.name() == called_cmd_name
            && passed_flags.iter().all(|passed_flag| {
                self.signature().flags.iter().any(|flag_decl| {
                    flag_decl.is_required()
                        && match passed_flag {
                            FlagVariant::LongFlag(name) => flag_decl
                                .long_name
                                .as_ref()
                                .map(|flag_decl_name| flag_decl_name == name)
                                .unwrap_or(false),
                            FlagVariant::ShortFlag(name) => flag_decl
                                .short_name
                                .as_ref()
                                .map(|flag_decl_name| flag_decl_name == name)
                                .unwrap_or(false),
                        }
                })
            })
    }
}

// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait CommandClone {
    fn clone_box(&self) -> Rc<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
    fn clone_box(&self) -> Rc<dyn Command> {
        Rc::new(self.clone())
    }
}

// // We can now implement Clone manually by forwarding to clone_box.
// impl Clone for Rc<dyn Command> {
//     fn clone(&self) -> Rc<dyn Command> {
//         self.clone_box()
//     }
// }

impl Into<Variable> for Rc<dyn Command> {
    fn into(self) -> Variable {
        let name = self.name().to_string();
        let decl = self.signature().decl.clone();
        let value = Value::new_func(self);
        Variable::new(name, value, VarDeclNode::CatchAll(decl))
    }
}
