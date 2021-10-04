mod push;
use lu_error::LuResult;
use push::ArrayPushCmd;

use lu_interpreter_structs::{Command, Scope, Variable};
use vec_box::vec_box;

pub fn source_array_module(_: &[&str], scope: &mut Scope<Variable>) -> LuResult<()> {
    let cmds: Vec<Box<dyn Command>> = vec_box![ArrayPushCmd::new()];
    for cmd in cmds {
        scope.cur_mut_frame().insert_var(cmd.into());
    }

    Ok(())
}
