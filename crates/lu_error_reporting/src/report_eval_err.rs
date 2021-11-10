use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::EvalErr;

use crate::{byte_range_of_item, f_id_of_item};

pub(crate) fn eval_err_to_diagnostic(err: &EvalErr) -> Diagnostic<usize> {
    match err {
        EvalErr::Message(m) => Diagnostic::error().with_message(m).with_code("E-Ast0001"),
        EvalErr::VarNotFound(_) => {
            todo!("VarNotFound Should not come up in eval")
        }
        EvalErr::NotConvertibleToBool(_) => {
            todo!("This error should be caught in ty checking");
        }
        EvalErr::SpawningExternalProcessFailed(term, err_message) => Diagnostic::error()
            .with_message(&format!(
                "Could not spawn external command: {}",
                err_message
            ))
            .with_code("E-Eval0002")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term),
                byte_range_of_item(&(term)),
            )
            .with_message("External command called here")]),
        EvalErr::ExternalCmdStdinWriteErr(term, err_message) => Diagnostic::error()
            .with_message(&format!(
                "Could not write to stdin of external cmd: {}",
                err_message
            ))
            .with_code("E-Eval0004")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term),
                byte_range_of_item(&(term)),
            )
            .with_message("External command called here")]),
        EvalErr::ExternalCmdStdoutReadErr(term, err_message) => Diagnostic::error()
            .with_message(&format!(
                "Could not read from stdout of external cmd: {}",
                err_message
            ))
            .with_code("E-Eval0005")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term),
                byte_range_of_item(&(term)),
            )
            .with_message("External command called here")]),
        EvalErr::ExternalCmdFailed(term) => Diagnostic::error()
            .with_message("External command failed")
            .with_code("E-Eval0006")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term),
                byte_range_of_item(&(term)),
            )
            .with_message("External command called here")]),
        EvalErr::DbgAbort => Diagnostic::note().with_message("Abort through user intervention"),
    }
}
