use std::sync::Arc;

use crate::cmd_prelude::*;
use lu_error::EvalErr;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct FsLsCmd {
    sign: Signature,
}

const PATHS_ARG_NAME: &str = "paths";
static LS_CMD_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

const LS_ENTRY_STRCT_NAME: &str = "LsEntry";
const LS_ENTRY_FIELD_0: &str = "Name";
const LS_ENTRY_FIELD_1: &str = "Type";
const LS_ENTRY_FIELD_2: &str = "Size";

pub(crate) static LS_ENTRY_STRCT: Lazy<Arc<RwLock<Strct>>> = Lazy::new(|| {
    let decl = lu_source_code_item!();
    let ls_entry_strct = Strct::new(
        LS_ENTRY_STRCT_NAME.into(),
        vec![
            StrctField::new(LS_ENTRY_FIELD_0.into(), ValueType::String, 0, decl.clone()),
            StrctField::new(LS_ENTRY_FIELD_1.into(), ValueType::String, 1, decl.clone()),
            StrctField::new(LS_ENTRY_FIELD_2.into(), ValueType::String, 1, decl.clone()),
        ],
        decl,
    );
    Arc::new(RwLock::new(ls_entry_strct))
});

impl FsLsCmd {
    pub fn new() -> Self {
        let ls_decl = lu_source_code_item!();
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(ls_decl.clone())
            .var_arg(ArgSignature::new(
                PATHS_ARG_NAME.to_string(),
                ValueType::String,
                ls_decl.clone().into(),
            ))
            .ret_arg(ArgSignature::new(
                "LsTable".into(),
                ValueType::Strct(Arc::downgrade(&*LS_ENTRY_STRCT)),
                ls_decl.clone().into(),
            ));

        FsLsCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for FsLsCmd {
    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        Some(&super::FS_MOD_PATH)
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*LS_CMD_ATTRS
    }

    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
        let mut entries = vec![];

        let pwd = scope
            .lock()
            .find_var("PWD")
            .map(|var| var.val.as_string())
            .flatten()
            .expect("pwd always string")
            .clone();
        assert!(!pwd.ends_with("/"));

        let pattern = "*"; // TODO take pattern as arg

        let glob_pattern = format!("{}/{}", pwd, pattern);
        debug!("ls {}", glob_pattern);
        let paths = glob::glob(&glob_pattern).map_err(|e| EvalErr::Message(e.to_string()))?;

        for path in paths {
            let path = path.map_err(|e| EvalErr::Message(e.to_string()))?;
            let path_name = path
                .display()
                .to_string()
                .strip_prefix(&format!("{}/", pwd))
                .expect("Pattern includes pwd")
                .to_string();
            let md = path
                .metadata()
                .map_err(|e| EvalErr::Message(e.to_string()))?;

            let path_type = if md.file_type().is_dir() {
                "Directory"
            } else if md.file_type().is_file() {
                "File"
            } else {
                "Symlink"
            }
            .to_string();

            entries.push(Value::new_strct(
                LS_ENTRY_STRCT_NAME.into(),
                vec![
                    (LS_ENTRY_FIELD_0.into(), path_name.into()),
                    (LS_ENTRY_FIELD_1.into(), path_type.into()),
                    (LS_ENTRY_FIELD_2.into(), md.len().into()),
                ],
            ))
        }

        Ok(Value::new_array(entries))
    }
}
