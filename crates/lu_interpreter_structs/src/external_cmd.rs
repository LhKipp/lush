use crate::{CmdAttribute, SyScope, Value, external_cmds_attr::{EXT_CMDS_ATTRIBUTES, EXT_CMDS_DEF_ATTRIBUTES}};
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};
use std::{io::Write, process::Stdio};

use crate::external_cmd;
use lu_error::{lu_source_code_item, EvalErr, LuResult, SourceCodeItem};
use once_cell::unsync::OnceCell;

use crate::{ArgSignature, Command, Signature, ValueType};

pub fn external_cmd_signature() -> Signature {
    let lu_item = lu_source_code_item!();
    Signature::new(
        Vec::new(),
        Some(ArgSignature::req(
            VAR_ARGS_DEF_NAME.into(),
            ValueType::Any,
            lu_item.clone(),
        )),
        Vec::new(),
        ArgSignature::req(IN_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
        ArgSignature::req(RET_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
        lu_item,
    )
}

#[derive(Debug, Clone, new)]
pub struct RunExternalCmd {
    /// The node in the AST which is evaluated by this Command
    pub cmd_node: SourceCodeItem,
    pub cmd_name: String,
    // TODO this could be global..., but would need adaptation of the signature() method from
    // Command
    #[new(default)]
    signature: OnceCell<Signature>,
}

impl Command for RunExternalCmd {
    fn name(&self) -> &str {
        &self.cmd_name
    }

    fn signature(&self) -> &Signature {
        self.signature
            .get_or_init(|| external_cmd::external_cmd_signature())
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!() // TODO fixup line number
    }

    fn parent_module(&self) -> Option<&crate::ModPath> {
        None
    }

    fn attributes(&self) -> &[CmdAttribute] {
        EXT_CMDS_ATTRIBUTES
            .get(self.name())
            .map(|attrs| attrs.as_ref())
            .unwrap_or(&EXT_CMDS_DEF_ATTRIBUTES)
    }

    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
        let l_scope = scope.lock();

        let args = self.expect_args(
            &self
                .signature()
                .var_arg
                .as_ref()
                .expect("ExternalCmd has vararg")
                .name,
            &l_scope,
        );

        // Historic shells expand wildcards (*, **) to all files matching the pattern in the
        // current PWD. Lush doesn't do the same automatically for internal cmds. For better
        // compatability, we now expand filenames
        let mut args_as_str = vec![];
        for arg in &**args {
            if let Value::FileName(f_name) = arg {
                match glob::glob(&f_name) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(path) => args_as_str.push(path.display().to_string()),
                                Err(e) => {
                                    return Err(EvalErr::Message(e.to_string()).into());
                                }
                            }
                        }
                    }
                    Err(e) => unreachable!("TODO check all globs are valid: {}", e),
                }
            } else {
                args_as_str.push(arg.to_string())
            }
        }

        let args = args_as_str;
        let stdin = self.get_in(&l_scope).cloned().unwrap_or(Value::Nil);

        let mut child = std::process::Command::new(self.cmd_name.clone())
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                EvalErr::SpawningExternalProcessFailed(self.cmd_node.clone(), e.to_string())
            })?;

        if !stdin.is_nil() {
            child
                .stdin
                .as_mut()
                .expect("Cmd stdin always correctly captured :)")
                .write_all(stdin.to_string().as_bytes())
                .map_err(|e| {
                    EvalErr::ExternalCmdStdinWriteErr(self.cmd_node.clone(), format!("{:?}", e))
                })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            EvalErr::ExternalCmdStdoutReadErr(self.cmd_node.clone(), format!("{:?}", e))
        })?;

        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout)?;
            Ok(Value::BareWord(raw_output))
        } else {
            Err(EvalErr::ExternalCmdFailed(self.cmd_node.clone()).into())
        }
    }
}
