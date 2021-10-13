mod push;
use std::rc::Rc;

use lu_error::{lu_source_code_item, SourceCodeItem};
use lu_text_util::{lu_source_code, SourceCode};
use once_cell::sync::Lazy;
use push::ArrayPushCmd;

use lu_interpreter_structs::prelude::*;
use vec_rc::vec_rc;

use super::LuRustStdMod;

static ARRAY_MOD_PATH: Lazy<ModPath> = Lazy::new(|| {
    ModPath::new(
        vec!["std".to_string(), "array".to_string()],
        ModPathVariant::StdPath,
    )
});

pub(crate) struct StdArrayMod {}

impl LuRustStdMod for StdArrayMod {
    fn id(&self) -> Vec<String> {
        ARRAY_MOD_PATH.parts.clone()
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
