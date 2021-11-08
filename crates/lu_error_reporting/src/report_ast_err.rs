use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::AstErr;

use crate::{f_id_of_item, SFAddrToFileMap};

pub(crate) fn ast_err_to_diagnostic(
    err: &AstErr,
    sf_node_addr_to_file_id: &SFAddrToFileMap,
) -> Diagnostic<usize> {
    match err {
        AstErr::Message(m) => Diagnostic::error().with_message(m).with_code("E-Ast0001"),
        AstErr::VarNotInScope(var_usage) => Diagnostic::error()
            .with_message("Variable not in scope")
            .with_code("E-Ast0002")
            .with_labels(vec![Label::primary(
                f_id_of_item(&var_usage, sf_node_addr_to_file_id),
                var_usage.range,
            )
            .with_message("Variable not found")]),
        AstErr::StrctNotInScope(strct_usage) => Diagnostic::error()
            .with_message("Struct not in scope")
            .with_code("E-Ast0003")
            .with_labels(vec![Label::primary(
                f_id_of_item(&strct_usage, sf_node_addr_to_file_id),
                strct_usage.range,
            )
            .with_message("Struct not found")]),
        AstErr::CmdNotInScope(cmd_usage) => Diagnostic::error()
            .with_message("Command not in scope")
            .with_code("E-Ast0004")
            .with_labels(vec![Label::primary(
                f_id_of_item(&cmd_usage, sf_node_addr_to_file_id),
                cmd_usage.range,
            )
            .with_message("Command not found")]),
        AstErr::CantUseRelativeInclude(_) => todo!(),
    }
}
