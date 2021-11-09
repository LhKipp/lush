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

pub(crate) static LS_ENTRY_STRCT: Lazy<Arc<RwLock<Strct>>> = Lazy::new(|| {
    let decl = lu_source_code_item!();
    let ls_entry_strct = Strct::new(
        LS_ENTRY_STRCT_NAME.into(),
        vec![StrctField::new(
            LS_ENTRY_FIELD_0.into(),
            ValueType::String,
            0,
            decl.clone(),
        )],
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

    fn do_run_cmd(&self, _: &mut SyScope) -> LuResult<Value> {
        let mut entries = vec![];
        let paths = glob::glob("*").map_err(|e| EvalErr::Message(e.to_string()))?;

        for path in paths {
            let path = path.map_err(|e| EvalErr::Message(e.to_string()))?;
            entries.push(Value::new_strct(
                LS_ENTRY_STRCT_NAME.into(),
                vec![(
                    LS_ENTRY_FIELD_0.into(),
                    path.to_string_lossy().to_string().into(),
                )],
            ))
        }

        Ok(Value::new_array(entries))
    }
}
