use codespan_reporting::diagnostic::{Diagnostic, Label};
use lu_error::ParseErr;

use crate::{byte_range_of_item, f_id_of_item};

pub(crate) fn parse_err_to_diagnostic(err: &ParseErr) -> Diagnostic<usize> {
    match err {
        ParseErr::MessageAt(..) => unreachable!("Should be mapped to MessageAtItem"),
        ParseErr::MessageAtItem(msg, item) => Diagnostic::error()
            .with_message(msg)
            .with_code("E-Parse0001")
            .with_labels(vec![Label::primary(
                f_id_of_item(&item),
                byte_range_of_item(&(item)),
            )]),
    }
}
