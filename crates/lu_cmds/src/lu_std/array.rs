mod push;
use std::rc::Rc;

use lu_error::{lu_source_code_item, SourceCodeItem};
use lu_text_util::{lu_source_code, SourceCode};
use push::ArrayPushCmd;

use lu_interpreter_structs::prelude::*;
use vec_rc::vec_rc;

use super::LuRustStdMod;

pub(crate) struct StdArrayMod {}

impl LuRustStdMod for StdArrayMod {
    fn id(&self) -> Vec<String> {
        vec!["std".to_string(), "array".to_string()]
    }
    fn rust_decl(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }
    fn rust_src(&self) -> SourceCode {
        lu_source_code!()
    }

    fn uses(&self) -> Vec<ModPath> {
        vec![]
    }
    fn cmds(&self) -> Vec<Rc<dyn Command>> {
        vec_rc![ArrayPushCmd::new()]
    }
}
