mod ls;

use crate::{
    cmd_prelude::*,
    lu_std::fs::ls::{FsLsCmd, LS_ENTRY_STRCT},
};

use super::LuRustStdMod;

static FS_MOD_PATH: Lazy<ModPath> = Lazy::new(|| ModPath::StdPath("std:fs".into()));

pub(crate) struct StdFsMod {}

impl LuRustStdMod for StdFsMod {
    fn id(&self) -> String {
        FS_MOD_PATH.as_std_path().unwrap().clone()
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
        vec_rc![FsLsCmd::new()]
    }

    fn strcts(&self) -> Vec<std::sync::Arc<parking_lot::RwLock<Strct>>> {
        vec![LS_ENTRY_STRCT.clone()]
    }
}
