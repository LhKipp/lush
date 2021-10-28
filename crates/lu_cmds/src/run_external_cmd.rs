use crate::external_cmds_attr::EXT_CMDS_ATTRIBUTES;
use crate::{cmd_prelude::*, external_cmds_attr::EXT_CMDS_DEF_ATTRIBUTES};
use std::{io::Write, process::Stdio};

use lu_error::{lu_source_code_item, EvalErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::{ArgSignature, ValueType};
use lu_syntax::{ast::CmdStmtNode, AstNode};
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};
use once_cell::unsync::OnceCell;

#[derive(Debug, Clone, new)]
pub struct RunExternalCmd {
    /// The node in the AST which is evaluated by this Command
    pub cmd_node: CmdStmtNode,
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
        self.signature.get_or_init(|| {
            let lu_item = lu_source_code_item!();
            let sign = Signature::new(
                Vec::new(),
                Some(ArgSignature::new(
                    VAR_ARGS_DEF_NAME.into(),
                    ValueType::Any,
                    lu_item.clone(),
                )),
                Vec::new(),
                ArgSignature::new(IN_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
                ArgSignature::new(RET_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
                lu_item,
            );
            sign
        })
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!() // TODO fixup line number
    }

    fn parent_module(&self) -> Option<&lu_interpreter_structs::ModPath> {
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

        let args = self.expect_args(&l_scope);
        let args: Vec<String> = args.iter().map(Value::to_string).collect();
        let stdin = self.get_in(&l_scope).cloned().unwrap_or(Value::Nil);

        let mut child = std::process::Command::new(self.cmd_name.clone())
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                EvalErr::SpawningExternalProcessFailed(self.cmd_node.to_item(), format!("{:?}", e))
            })?;

        if !stdin.is_nil() {
            child
                .stdin
                .as_mut()
                .expect("Cmd stdin always correctly captured :)")
                .write_all(stdin.to_string().as_bytes())
                .map_err(|e| {
                    EvalErr::ExternalCmdStdinWriteErr(self.cmd_node.to_item(), format!("{:?}", e))
                })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            EvalErr::ExternalCmdStdoutReadErr(self.cmd_node.to_item(), format!("{:?}", e))
        })?;

        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout)?;
            Ok(Value::BareWord(raw_output))
        } else {
            Err(EvalErr::ExternalCmdFailed(self.cmd_node.to_item()).into())
        }
    }
}
