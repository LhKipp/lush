use std::path::{Path, PathBuf};

use lu_error::EvalErr;

use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct CdBuiltin {
    sign: Signature,
}

const CD_INTO_DIR_ARG: &str = "directory";
static CD_BUILTIN_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl CdBuiltin {
    pub fn new() -> Self {
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(lu_source_code_item!())
            .args(vec![ArgSignature::opt(
                CD_INTO_DIR_ARG.to_string(),
                ValueType::FileName,
                lu_source_code_item!(-3).into(),
            )]);
        CdBuiltin {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for CdBuiltin {
    fn name(&self) -> &str {
        "cd"
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        None
    }

    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
        let mut l_scope = scope.lock();
        let dir_arg = l_scope.find_var(CD_INTO_DIR_ARG).unwrap();
        let (path, decl_item): (PathBuf, SourceCodeItem) =
            if let Some(dir_arg_some_val) = dir_arg.val.expect_optional_inner_val() {
                (
                    dir_arg_some_val.coerce_to_filename().unwrap().into(),
                    dir_arg.decl.to_item(),
                )
            } else {
                // Cd into home dir
                if let Some(home) = l_scope.find_var("HOME") {
                    (
                        home.val
                            .as_file_name()
                            .expect("HOME is always FileName")
                            .into(),
                        home.decl.to_item(),
                    )
                } else {
                    return Err(EvalErr::Message("Uups. $HOME is not set.".into()).into());
                }
            };

        let pwd = l_scope.find_var_mut("PWD").expect("PWD always set");

        let path = if path.is_absolute() {
            path
        } else {
            let pwd: PathBuf = pwd.val.as_file_name().unwrap().into();
            pwd.join(path)
        };

        if !path.is_dir() {
            return Err(EvalErr::PathIsNotDirectory {
                path_item: decl_item,
                path: path.display().to_string(),
            }
            .into());
        }

        match fs_err::canonicalize(path) {
            Err(e) => {
                return EvalErr::Message(e.to_string()).into();
            }
            Ok(p) => {
                let p_as_ref: &Path = p.as_ref();
                if let Err(e) = std::env::set_current_dir(p_as_ref) {
                    return Err(EvalErr::Message(format!(
                        "Could not cd to {}: {}",
                        p.display(),
                        e
                    ))
                    .into());
                }
                pwd.set_val(Value::FileName(p.display().to_string()))?;
            }
        }

        Ok(Value::Nil)
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*CD_BUILTIN_ATTRS
    }
}

#[cfg(test)]
mod tests {
    use lu_interpreter::Interpreter;
    use lu_test_support::test_prelude::*;
    use lu_text_util::SourceCode;

    #[test]
    fn cd_into_dir_works() {
        let playground = Playground::new().permanent();

        playground.make_dirs("dir_a");
        let (global_frame, itprt_cfg) = make_test_interpreter_in_playground(playground);

        let eval_result = Interpreter::eval_for_tests(
            SourceCode::new_text(
                r#"
                cd dir_a
                $PWD
            "#
                .to_string(),
            ),
            global_frame,
            &itprt_cfg,
        );
        assert!(eval_result.is_ok(), "{:?}", eval_result);
        let val = eval_result.unwrap().to_string();
        assert!(val.ends_with("/dir_a"), "PWD {} not ending with dir_a", val);
    }
}
