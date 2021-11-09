use std::fmt::Debug;
use std::rc::Rc;

use crate::{
    FlagVariant, Function, ModPath, Scope, Signature, SyScope, Value, VarDeclNode, Variable,
};

use log::debug;
use lu_error::{LuResult, SourceCodeItem};
use serde::{Deserialize, Serialize};

pub const IN_VAR_NAME: &str = "in";
pub const ARGS_VAR_NAME: &str = "args";

#[derive(Debug, Clone, Serialize, Deserialize, is_enum_variant, PartialEq, Eq)]
pub enum CmdAttributeVariant {
    Pure,
    Impure,
    PurityUnknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct CmdAttribute {
    attr: CmdAttributeVariant,
    decl: SourceCodeItem,
}

impl From<(CmdAttributeVariant, SourceCodeItem)> for CmdAttribute {
    fn from((attr, decl): (CmdAttributeVariant, SourceCodeItem)) -> Self {
        CmdAttribute::new(attr, decl)
    }
}

pub trait Command: CommandClone + Debug {
    // Methods to be overwritten by a cmd

    /// The name of the cmd
    fn name(&self) -> &str;

    fn signature(&self) -> &Signature;

    /// Returns SourceCodeItem into the signature/declaration of the command
    fn signature_item(&self) -> SourceCodeItem;

    /// Returns the module, in which the command is contained / declared
    /// TODO this should be always Some (even for print, ...)
    fn parent_module(&self) -> Option<&ModPath>;

    /// Get all attributes of this command.
    /// There might be no attributes returned for external cmds. Therefore the absence
    /// of an attribute, does not necessarily imply, that the command does not have the guarantees
    /// an attribute implies.
    fn attributes(&self) -> &[CmdAttribute];

    /// Run the cmd
    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value>;

    // End of methods to overwrite

    /// Convenience func for users
    fn find_attr(&self, var: CmdAttributeVariant) -> Option<&CmdAttribute> {
        self.attributes().iter().find(|attr| attr.attr == var)
    }

    /// Only overwritten by Function. Overwritting this func for anything else than Function is an
    /// error
    fn as_function(&self) -> Option<&Function> {
        None
    }

    fn as_function_mut(&mut self) -> Option<&mut Function> {
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
    fn get_in<'a>(&self, scope: &'a Scope<Variable>) -> Option<&'a Value> {
        self.get_arg(scope, IN_VAR_NAME)
    }

    /// Returns $in
    fn expect_in<'a>(&self, scope: &'a Scope<Variable>) -> &'a Value {
        self.expect_arg(scope, IN_VAR_NAME)
    }

    /// Returns $<arg_name>
    fn get_arg<'a>(&self, scope: &'a Scope<Variable>, arg_name: &str) -> Option<&'a Value> {
        scope.find_var(arg_name).map(|var| &var.val)
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

    /// Takes the contents of the vararg
    /// TODO I was overeager optimizing. In case someone passes an array to the vararg this doesn't
    /// work (not possible currently)
    /// print $my_array..
    fn take_var_arg<'a>(&self, scope: &'a mut Scope<Variable>, var_arg_name: &str) -> Vec<Value> {
        scope
            .find_var_mut(var_arg_name)
            .map(|var| std::mem::replace(&mut var.val, Value::Nil))
            .map(|val| {
                if let Value::Array(arr) = val {
                    Rc::try_unwrap(arr).expect("Must work")
                } else {
                    unreachable!("Var arg is always an array");
                }
            })
            .expect("Variable always present")
    }

    /// Returns $<arg_name>
    // fn expect_overwrite_var<'a>(
    //     &self,
    //     scope: &'a mut Scope<Variable>,
    //     var_name: &str,
    //     new_val: Value,
    // ) {
    //     assert!(&scope.overwrite_var_value(var_name, new_val))
    // }

    fn run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
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
        // TODO is this logic correct?
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
