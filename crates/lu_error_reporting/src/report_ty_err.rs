use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::TyErr;

use crate::{byte_range_of_item, f_id_of_item};
pub(crate) fn ty_err_to_diagnostic(err: &TyErr) -> Diagnostic<usize> {
    match err {
        TyErr::Message(m) => Diagnostic::error().with_message(m).with_code("E-Ty0001"),
        TyErr::TermDoesNotReturnType(term) => Diagnostic::error()
            .with_message("Statement does not return value")
            .with_code("E-Ty0002")
            .with_labels(vec![Label::primary(
                f_id_of_item(&term),
                byte_range_of_item(&(term)),
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
                .with_message("Type mismatch")
                .with_code("E-Ty0003")
                .with_labels(vec![
                    Label::secondary(f_id_of_item(&lhs_decl), byte_range_of_item(&(lhs_decl)))
                        .with_message(format!("Found to be of type: {}", lhs_ty)),
                    Label::secondary(f_id_of_item(&rhs_decl), byte_range_of_item(&(rhs_decl)))
                        .with_message(format!("Found to be of type: {}", rhs_ty)),
                ])
        }
        TyErr::UnexpectedArg { arg, fn_decl } => Diagnostic::error()
            .with_message("Unexpected Argument")
            .with_code("E-Ty0004")
            .with_labels(vec![
                Label::primary(f_id_of_item(&arg), byte_range_of_item(&(arg)))
                    .with_message("Not expected according to fn decl"),
                Label::secondary(f_id_of_item(&fn_decl), byte_range_of_item(&(fn_decl)))
                    .with_message("Function declaration"),
            ]),
        TyErr::UnsatisfiedArg { arg_decl, cmd_stmt } => Diagnostic::error()
            .with_message("Argument not passed")
            .with_code("E-Ty0005")
            .with_labels(vec![
                Label::primary(f_id_of_item(&arg_decl), byte_range_of_item(&(arg_decl)))
                    .with_message("Argument declared here"),
                Label::secondary(f_id_of_item(&cmd_stmt), byte_range_of_item(&(cmd_stmt)))
                    .with_message("Command called here"),
            ]),
        TyErr::VarExpectedToBeFunc { var_usage } => Diagnostic::error()
            .with_message("Variable expected to be a command")
            .with_code("E-Ty0006")
            .with_labels(vec![Label::primary(
                f_id_of_item(&var_usage),
                byte_range_of_item(&(var_usage)),
            )
            .with_message("Variable used here as if it would be a command")]),
        TyErr::ItemExpectedToBeFunc(item) => Diagnostic::error()
            .with_message("Statement expected to be a command")
            .with_code("E-Ty0007")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
            )
            .with_message("Item declared here")]),
        TyErr::ItemExpectedToBeStruct(item) => Diagnostic::error()
            .with_message("Statement expected to be a struct")
            .with_code("E-Ty0008")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
            )
            .with_message("Item declared here")]),
        TyErr::ItemExpectedToBeArray(item) => Diagnostic::error()
            .with_message("Statement expected to be an array")
            .with_code("E-Ty0008")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
            )
            .with_message("Item declared here")]),
        TyErr::ItemExpectedToBeOptional(item) => Diagnostic::error()
            .with_message("Statement expected to be an optional")
            .with_code("E-Ty0009")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
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
                Label::primary(f_id_of_item(&usage), byte_range_of_item(&(usage)))
                    .with_message("Wrong struct usage here"),
                Label::secondary(f_id_of_item(&strct_decl), byte_range_of_item(&(strct_decl)))
                    .with_message("Struct declared here"),
            ]),
        TyErr::FlagWithoutArgument(flag) => Diagnostic::error()
            .with_message("Flag used without providing the necessary argument")
            .with_code("E-Ty0040")
            .with_labels(vec![Label::primary(
                f_id_of_item(&flag),
                byte_range_of_item(&(flag)),
            )
            .with_message("Flag passed here")]),
        TyErr::PassingOfNotDeclaredFlag(flag_usage) => Diagnostic::error()
            .with_message("Flag passed, but not declared")
            .with_code("E-Ty0041")
            .with_labels(vec![Label::primary(
                f_id_of_item(&flag_usage),
                byte_range_of_item(&(flag_usage)),
            )
            .with_message("Flag passed here")]),
        TyErr::NotPassedRequiredFlag {
            flag_decl,
            cmd_stmt,
        } => Diagnostic::error()
            .with_message("A required flag was not passed")
            .with_code("E-Ty0042")
            .with_labels(vec![
                Label::primary(f_id_of_item(&cmd_stmt), byte_range_of_item(&(cmd_stmt)))
                    .with_message("Command called here"),
                Label::secondary(f_id_of_item(&flag_decl), byte_range_of_item(&(flag_decl)))
                    .with_message("Flag declared here"),
            ]),
        TyErr::TableRowToManyCol { row } => Diagnostic::error()
            .with_message("The table row contains to many values")
            .with_code("E-Ty0050")
            .with_labels(vec![Label::primary(
                f_id_of_item(&row),
                byte_range_of_item(&(row)),
            )]),

        TyErr::TableRowToFewCol { row } => Diagnostic::error()
            .with_message("The table row contains to few values")
            .with_code("E-Ty0051")
            .with_labels(vec![Label::primary(
                f_id_of_item(&row),
                byte_range_of_item(&(row)),
            )]),
        TyErr::ExpectedStmtToBeInferred { stmt } => Diagnostic::error()
            .with_message("Expected to be able to infer the type behind the statement")
            .with_code("E-Ty0060")
            .with_labels(vec![Label::primary(
                f_id_of_item(&stmt),
                byte_range_of_item(&(stmt)),
            )
            .with_message("Hint: Try adding type annotations")]),
        TyErr::ExpectedStmtToReturnAnArrayOfStrcts {
            stmt_with_wrong_ret,
            found_ty,
        } => Diagnostic::error()
            .with_message("select's input must be a table")
            .with_code("E-Ty0061")
            .with_labels(vec![Label::primary(
                f_id_of_item(&stmt_with_wrong_ret),
                byte_range_of_item(&(stmt_with_wrong_ret)),
            )
            .with_message(format!("Returns: {}", found_ty))]),
        TyErr::SelectArgMustBeBareWordOrString { arg } => Diagnostic::error()
            .with_message("selected column name must be a bareword or a string")
            .with_code("E-Ty0062")
            .with_labels(vec![Label::primary(
                f_id_of_item(&arg),
                byte_range_of_item(&arg),
            )]),
    }
}
