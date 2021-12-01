use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::cmd_prelude::*;
use glob::Paths;
use lu_error::EvalErr;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct FsLsCmd {
    sign: Signature,
}

const PATHS_VAR_ARG_NAME: &str = "paths";
static LS_CMD_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

const LS_ENTRY_STRCT_NAME: &str = "LsEntry";
const LS_ENTRY_FIELD_0: &str = "name";
const LS_ENTRY_FIELD_1: &str = "type";
const LS_ENTRY_FIELD_2: &str = "size";

pub(crate) static LS_ENTRY_STRCT: Lazy<Arc<RwLock<Strct>>> = Lazy::new(|| {
    let decl = lu_source_code_item!(-1);
    let ls_entry_strct = Strct::new(
        LS_ENTRY_STRCT_NAME.into(),
        vec![
            StrctField::new(LS_ENTRY_FIELD_0.into(), ValueType::String, 0, decl.clone()),
            StrctField::new(LS_ENTRY_FIELD_1.into(), ValueType::String, 1, decl.clone()),
            StrctField::new(LS_ENTRY_FIELD_2.into(), ValueType::Number, 2, decl.clone()),
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
            .var_arg(ArgSignature::req(
                PATHS_VAR_ARG_NAME.to_string(),
                ValueType::FileName,
                ls_decl.clone().into(),
            ))
            .ret_arg(ArgSignature::req(
                "LsTable".into(),
                ValueType::new_array(
                    ValueType::Strct(Arc::downgrade(&*LS_ENTRY_STRCT)),
                    lu_source_code_item!(),
                ),
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

        let mut l_scope = scope.lock();
        let (_, pwd) = get_pwd_var(&l_scope);
        let pwd = pwd.clone();

        let patterns = {
            let mut patterns: Vec<_> = self
                .expect_args(PATHS_VAR_ARG_NAME, &mut l_scope)
                .iter()
                .map(|pattern| pattern.coerce_to_filename().unwrap().to_string())
                .collect();
            if patterns.is_empty() {
                patterns.push("*".into())
            }
            patterns
        };
        debug!("Found pwd: {} and patterns {:?}", pwd, patterns);

        let matching_paths: Result<Vec<Paths>, EvalErr> = patterns
            .into_iter()
            .map(|pattern| {
                let glob_pattern = {
                    let path: PathBuf = format!("{}/{}", pwd, pattern).into();
                    if path.is_dir() {
                        format!("{}/*", path.display())
                    } else {
                        path.display().to_string()
                    }
                };

                if !glob_pattern.contains(|c| c == '*') && !Path::new(&glob_pattern).exists() {
                    // normal path, not containing any wildcards. Must exist
                    return Err(EvalErr::Message(format!(
                        "ls: cannot access '{}': No such file or directory",
                        glob_pattern
                    )));
                }

                glob::glob(&glob_pattern).map_err(|e| EvalErr::Message(e.to_string()))
            })
            .collect();
        let matching_paths: Result<Vec<PathBuf>, _> =
            matching_paths?.into_iter().flatten().collect();
        let matching_paths = matching_paths.map_err(|e| EvalErr::Message(e.to_string()))?;

        for path in matching_paths {
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
