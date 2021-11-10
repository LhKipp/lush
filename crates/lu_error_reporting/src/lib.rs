mod files;
mod lu_source_files;
mod lu_source_files_util;
mod report_ast_err;
mod report_eval_err;
mod report_parse_err;
mod report_ty_err;

use std::ops::Range;

use codespan_reporting::{
    diagnostic::Diagnostic,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use log::debug;
use lu_error::{LuErr, SourceCodeItem};
use lu_interpreter_structs::*;
use lu_source_files_util::find_file;

use crate::{
    files::DiagnosticFileContainer, report_ast_err::ast_err_to_diagnostic,
    report_eval_err::eval_err_to_diagnostic, report_parse_err::parse_err_to_diagnostic,
    report_ty_err::ty_err_to_diagnostic,
};

pub fn report_to_term(errors: &[LuErr], scope: &Scope<Variable>) -> Result<(), String> {
    debug!("Reporting {} errors to terminal", errors.len());
    let mut files = DiagnosticFileContainer::empty();

    for modi in scope
        .get_all_frames()
        .filter_map(|frame| frame.tag.as_module_frame())
    {
        if let Some(mod_address) = modi.mod_int_address() {
            files.add_file(mod_address, modi.id.to_string(), &modi.src.text);
        }
    }

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for diagnostic in errors.iter().map(|error| match error {
        LuErr::Parse(parse_err) => parse_err_to_diagnostic(parse_err),
        LuErr::Ty(ty_err) => ty_err_to_diagnostic(ty_err),
        LuErr::FS(fs_err) => Diagnostic::error()
            .with_message(fs_err.to_string())
            .with_code("File-IO error"),
        LuErr::Eval(eval_err) => eval_err_to_diagnostic(eval_err),
        LuErr::Ast(ast_err) => ast_err_to_diagnostic(ast_err),
        LuErr::Internal(msg) => Diagnostic::error()
            .with_message(msg)
            .with_code("Internal Error"),
    }) {
        term::emit(&mut writer.lock(), &config, &files, &diagnostic).map_err(|e| e.to_string())?
    }

    Ok(())
}

fn f_id_of_item(item: &SourceCodeItem) -> usize {
    if item.is_lu_source_code_item() {
        find_file(item.lu_source_code_file_name())
            .expect("Lu-Rust SourceCodeItem can always be found")
            .0
    } else {
        item.sf_node_addr
    }
}

fn byte_range_of_item(item: &SourceCodeItem) -> Range<usize> {
    if item.is_lu_source_code_item() {
        let line = item.lu_line();
        let content = find_file(item.lu_source_code_file_name())
            .expect("Lu-Rust SourceCodeItem can always be found")
            .1;
        // line to byte range
        let content_iter = content.as_bytes().iter().enumerate();
        let mut cur_line = 1;
        let mut content_iter = content_iter.skip_while(|(_, byte)| {
            if **byte == b'\n' {
                cur_line = cur_line + 1;
            }
            cur_line != line
        });
        let (line_start_byte, _) = content_iter.next().unwrap();
        let mut content_iter = content_iter.skip_while(|(_, byte)| **byte != b'\n');
        let line_end_byte = content_iter
            .next()
            .map(|(line_end_byte, _)| line_end_byte)
            .unwrap_or(content.as_bytes().len());

        line_start_byte..line_end_byte
    } else {
        item.range.into()
    }
}
