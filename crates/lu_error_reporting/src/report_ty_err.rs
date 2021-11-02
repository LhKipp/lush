use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::TyErr;

use crate::{f_id_of_item, SFAddrToFileMap};
pub(crate) fn ty_err_to_diagnostic(
    err: &TyErr,
    sf_node_addr_to_file_id: &SFAddrToFileMap,
) -> Diagnostic<usize> {
    match err {
        TyErr::Message(m) => Diagnostic::error().with_message(m).with_code("E-Ty0001"),
        TyErr::TermDoesNotReturnType(term) => Diagnostic::error()
            .with_message("Statement does not return value")
            .with_code("E-Ty0002")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term, sf_node_addr_to_file_id),
                term.range,
            )
            .with_message("This statement is used as if it would return a value")]),
        TyErr::TypesNotEqual {
            lhs_decl,
            lhs_ty,
            rhs_decl,
            rhs_ty,
        } => {
            let lhs_decl = lhs_decl.clone().unwrap();
            let rhs_decl = rhs_decl.clone().unwrap();
            Diagnostic::error()
                .with_message("Type mismach")
                .with_code("E-Ty0003")
                .with_labels(vec![
                    Label::secondary(
                        f_id_of_item(&lhs_decl, sf_node_addr_to_file_id),
                        lhs_decl.range,
                    )
                    .with_message(format!("Found to be of type: {}", lhs_ty)),
                    Label::secondary(
                        f_id_of_item(&rhs_decl, sf_node_addr_to_file_id),
                        rhs_decl.range,
                    )
                    .with_message(format!("Found to be of type: {}", rhs_ty)),
                ])
        }
        TyErr::UnexpectedArg { arg, fn_decl } => Diagnostic::error()
            .with_message("Unexpected Argument")
            .with_code("E-Ty0004")
            .with_labels(vec![
                Label::primary(f_id_of_item(&arg, sf_node_addr_to_file_id), arg.range)
                    .with_message("Not expected according to fn decl"),
                Label::secondary(
                    f_id_of_item(&fn_decl, sf_node_addr_to_file_id),
                    fn_decl.range,
                )
                .with_message("Function declaration"),
            ]),
        TyErr::UnsatisfiedArg { arg_decl, cmd_stmt } => Diagnostic::error()
            .with_message("Argument not passed")
            .with_code("E-Ty0005")
            .with_labels(vec![
                Label::primary(
                    f_id_of_item(&arg_decl, sf_node_addr_to_file_id),
                    arg_decl.range,
                )
                .with_message("Argument declared here"),
                Label::secondary(
                    f_id_of_item(&cmd_stmt, sf_node_addr_to_file_id),
                    cmd_stmt.range,
                )
                .with_message("Command called here"),
            ]),
        TyErr::VarExpectedToBeFunc { var_usage } => Diagnostic::error()
            .with_message("Variable expected to be a command")
            .with_code("E-Ty0006")
            .with_labels(vec![Label::primary(
                f_id_of_item(&var_usage, sf_node_addr_to_file_id),
                var_usage.range,
            )
            .with_message("Variable used here as if it would be a command")]),
        TyErr::ItemExpectedToBeFunc(item) => Diagnostic::error()
            .with_message("Statement expected to be a command")
            .with_code("E-Ty0007")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item, sf_node_addr_to_file_id),
                item.range,
            )
            .with_message("Item declared here")]),
        TyErr::ItemExpectedToBeStruct(item) => Diagnostic::error()
            .with_message("Statement expected to be a struct")
            .with_code("E-Ty0008")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item, sf_node_addr_to_file_id),
                item.range,
            )
            .with_message("Item declared here")]),
        TyErr::ItemExpectedToBeArray(item) => Diagnostic::error()
            .with_message("Statement expected to be a array")
            .with_code("E-Ty0008")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item, sf_node_addr_to_file_id),
                item.range,
            )
            .with_message("Item declared here")]),
        TyErr::StructDoesNotHaveField {
            field_name,
            strct_decl,
            usage,
        } => Diagnostic::error()
            .with_message(&format!("Struct does not contain field: {}", field_name))
            .with_code("E-Ty0030")
            .with_labels(vec![
                Label::primary(f_id_of_item(&usage, sf_node_addr_to_file_id), usage.range)
                    .with_message("Wrong struct usage here"),
                Label::secondary(
                    f_id_of_item(&strct_decl, sf_node_addr_to_file_id),
                    strct_decl.range,
                )
                .with_message("Struct declared here"),
            ]),
        TyErr::FlagWithoutArgument(flag) => Diagnostic::error()
            .with_message("Flag used without providing the necessary argument")
            .with_code("E-Ty0040")
            .with_labels(vec![Label::primary(
                f_id_of_item(&flag, sf_node_addr_to_file_id),
                flag.range,
            )
            .with_message("Flag passed here")]),
        TyErr::PassingOfNotDeclaredFlag(flag_usage) => Diagnostic::error()
            .with_message("Flag passed, but not declared")
            .with_code("E-Ty0041")
            .with_labels(vec![Label::primary(
                f_id_of_item(&flag_usage, sf_node_addr_to_file_id),
                flag_usage.range,
            )
            .with_message("Flag passed here")]),
        TyErr::NotPassedRequiredFlag {
            flag_decl,
            cmd_stmt,
        } => Diagnostic::error()
            .with_message("A required flag was not passed")
            .with_code("E-Ty0042")
            .with_labels(vec![
                Label::primary(
                    f_id_of_item(&cmd_stmt, sf_node_addr_to_file_id),
                    cmd_stmt.range,
                )
                .with_message("Command called here"),
                Label::secondary(
                    f_id_of_item(&flag_decl, sf_node_addr_to_file_id),
                    flag_decl.range,
                )
                .with_message("Flag declared here"),
            ]),
    }
}
