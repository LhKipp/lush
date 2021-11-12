use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::AstErr;

use crate::{byte_range_of_item, f_id_of_item};

pub(crate) fn ast_err_to_diagnostic(err: &AstErr) -> Diagnostic<usize> {
    match err {
        AstErr::Message(m) => Diagnostic::error().with_message(m).with_code("E-Ast0001"),
        AstErr::VarNotInScope(var_usage) => Diagnostic::error()
            .with_message("Variable not in scope")
            .with_code("E-Ast0002")
            .with_labels(vec![Label::primary(
                f_id_of_item(&var_usage),
                byte_range_of_item(&(var_usage)),
            )
            .with_message("Variable not found")]),
        AstErr::StrctNotInScope(strct_usage) => Diagnostic::error()
            .with_message("Struct not in scope")
            .with_code("E-Ast0003")
            .with_labels(vec![Label::primary(
                f_id_of_item(&strct_usage),
                byte_range_of_item(&(strct_usage)),
            )
            .with_message("Struct not found")]),
        AstErr::CmdNotInScope(cmd_usage) => Diagnostic::error()
            .with_message("Command not in scope")
            .with_code("E-Ast0004")
            .with_labels(vec![Label::primary(
                f_id_of_item(&cmd_usage),
                byte_range_of_item(&(cmd_usage)),
            )
            .with_message("Command not found")]),
        AstErr::CantUseRelativeInclude(item) => Diagnostic::error()
            .with_message("Cannot use a relative include here")
            .with_code("E-Ast00010")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
            )]),
        AstErr::PatternError { pattern, err } => Diagnostic::error()
            .with_message("Command not in scope")
            .with_code("E-Ast0004")
            .with_labels(vec![Label::primary(
                f_id_of_item(&pattern),
                byte_range_of_item(&pattern),
            )
            .with_message(err)]),
    }
}
