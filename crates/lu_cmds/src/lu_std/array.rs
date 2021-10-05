mod push;
use std::rc::Rc;

use lu_error::LuResult;
use push::ArrayPushCmd;

use lu_interpreter_structs::{Command, Scope, Variable};
use vec_rc::vec_rc;

pub fn source_array_module(_: &[&str], scope: &mut Scope<Variable>) -> LuResult<()> {
    let cmds: Vec<Rc<dyn Command>> = vec_rc![ArrayPushCmd::new()];
    for cmd in cmds {
        scope.cur_mut_frame().insert_var(cmd.into());
    }

    Ok(())
}
