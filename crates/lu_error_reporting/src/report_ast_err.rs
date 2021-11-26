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
        AstErr::ReqArgAfterOptionalArg { req_arg, opt_arg } => Diagnostic::error()
            .with_message("A required argument can not come after an optional one")
            .with_code("E-Ast0005")
            .with_labels(vec![
                Label::primary(f_id_of_item(&req_arg), byte_range_of_item(&req_arg))
                    .with_message("Required argument declared here"),
                Label::secondary(f_id_of_item(&opt_arg), byte_range_of_item(&opt_arg))
                    .with_message("Optional argument declared here"),
            ]),
        AstErr::VarArgAfterOptionalArg { var_arg, opt_arg } => Diagnostic::error()
            .with_message("A vararg argument can not come after an optional one")
            .with_code("E-Ast0006")
            .with_labels(vec![
                Label::primary(f_id_of_item(&var_arg), byte_range_of_item(&var_arg))
                    .with_message("Vararg argument declared here"),
                Label::secondary(f_id_of_item(&opt_arg), byte_range_of_item(&opt_arg))
                    .with_message("Optional argument declared here"),
            ]),
        AstErr::NoSuchStdPath { path_usage, .. } => Diagnostic::error()
            .with_message("No such standard module")
            .with_code("E-Ast0007")
            .with_labels(vec![Label::primary(
                f_id_of_item(&path_usage),
                byte_range_of_item(&path_usage),
            )]),
    }
}
