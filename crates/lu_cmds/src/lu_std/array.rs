mod push;
use std::rc::Rc;

use lu_error::{lu_source_code_item, SourceCodeItem};
use push::ArrayPushCmd;

use lu_interpreter_structs::prelude::*;
use vec_rc::vec_rc;

pub fn source_array_module(mod_path: &[String]) -> Vec<ScopeFrame<Variable>> {
    assert!(
        mod_path.is_empty(),
        "Can't source individual items from array"
    );
    let cmds: Vec<Rc<dyn Command>> = vec_rc![ArrayPushCmd::new()];
    let mut frame = ScopeFrame::new(ScopeFrameTag::new_source_file_tag(
        ModPath::new(
            vec!["std".to_string(), "array".to_string()],
            ModPathVariant::StdPath,
            lu_source_code_item!(),
        ),
        vec![],
    ));

    for cmd in cmds {
        frame.insert_var(Variable::new_func(cmd));
    }

    vec![frame]
}
