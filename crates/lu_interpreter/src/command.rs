use crate::Evaluable;

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

pub trait Command: Evaluable + CommandClone {
    fn name(&self) -> &str;
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
