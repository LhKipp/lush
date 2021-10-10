mod push;
use std::rc::Rc;

use lu_text_util::lu_source_code;
use push::ArrayPushCmd;

use lu_interpreter_structs::prelude::*;
use vec_rc::vec_rc;

pub fn source_array_module(mod_path: &[String]) -> Vec<ScopeFrame<Variable>> {
    assert!(
        mod_path.is_empty(),
        "Can't source individual items from array"
    );

    let source_code = lu_source_code!();
    let mod_path = ModPath::new(
        vec!["std".to_string(), "array".to_string()],
        ModPathVariant::StdPath,
    );
    let modi = ModInfo::new_std_module(mod_path, source_code, vec![]);

    let cmds: Vec<Rc<dyn Command>> = vec_rc![ArrayPushCmd::new()];
    let mut frame = ScopeFrame::new(ScopeFrameTag::ModuleFrame(modi));

    for cmd in cmds {
        frame.insert_var(Variable::new_func(cmd));
    }

    vec![frame]
}
