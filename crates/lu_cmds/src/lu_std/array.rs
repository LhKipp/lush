mod push;

use crate::cmd_prelude::*;
use push::ArrayPushCmd;

use vec_rc::vec_rc;

use super::LuRustStdMod;

static ARRAY_MOD_PATH: Lazy<ModPath> = Lazy::new(|| ModPath::StdPath("std:array".into()));

pub(crate) struct StdArrayMod {}

impl LuRustStdMod for StdArrayMod {
    fn id(&self) -> String {
        ARRAY_MOD_PATH.as_std_path().unwrap().clone()
    }
    fn rust_decl(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn uses(&self) -> Vec<ModPath> {
        vec![]
    }
    fn cmds(&self) -> Vec<Rc<dyn Command>> {
        vec_rc![ArrayPushCmd::new()]
    }

    fn strcts(&self) -> Vec<std::sync::Arc<parking_lot::RwLock<Strct>>> {
        vec![]
    }
}
