use std::{io::Write, process::Stdio};

use crate::{Command, EvalArg, Interpreter};
use lu_error::{EvalErr, LuResult};
use lu_syntax::{ast::CmdStmtNode, AstNode};
use lu_value::Value;

#[derive(Debug, Clone)]
pub struct RunExternalCmd {
    /// The node in the AST which is evaluated by this Command
    pub cmd_node: CmdStmtNode,
    pub cmd_name: String,
}

impl Command for RunExternalCmd {
    fn name(&self) -> &str {
        &self.cmd_name
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Interpreter) -> LuResult<Value> {
        let l_scope = state.scope.lock();

        let args = self.expect_args(&l_scope);
        let args: Vec<String> = args.iter().map(Value::to_string).collect();
        let stdin = self.expect_in(&l_scope);

        let mut child = std::process::Command::new(self.cmd_name.clone())
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| {
                EvalErr::SpawningExternalProcessFailed(
                    self.cmd_node.into_item(),
                    format!("{:?}", e),
                )
            })?;

        if !stdin.is_nil() {
            child
                .stdin
                .as_mut()
                .expect("Cmd stdin always correctly captured :)")
                .write_all(stdin.to_string().as_bytes())
                .map_err(|e| {
                    EvalErr::ExternalCmdStdinWriteErr(self.cmd_node.into_item(), format!("{:?}", e))
                })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            EvalErr::ReadingStdoutFromCmdFailed(self.cmd_node.into_item(), format!("{:?}", e))
        })?;

        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout)?;
            Ok(Value::BareWord(raw_output))
        } else {
            Err(EvalErr::ExternalCmdFailed(self.cmd_node.into_item()).into())
        }
    }
}
