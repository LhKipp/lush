mod report_ast_err;
mod report_eval_err;
mod report_ty_err;

use codespan_reporting::{
    diagnostic::Diagnostic,
    files::SimpleFiles,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use lu_error::{LuErr, ParseErr, SourceCodeItem};
use lu_interpreter_structs::*;
use lu_syntax::{ast, AstNode};
use std::collections::HashMap;

use crate::{
    report_ast_err::ast_err_to_diagnostic, report_eval_err::eval_err_to_diagnostic,
    report_ty_err::ty_err_to_diagnostic,
};

// use codespan_reporting::SimpleFiles;

pub(crate) type SFAddrToFileMap = HashMap<usize, usize>;
pub fn report_to_term(errors: &Vec<LuErr>, scope: &Scope<Variable>) -> Result<(), String> {
    let mut files = SimpleFiles::new();
    let mut sf_node_addr_to_file_id = HashMap::new();

    for modi in scope
        .get_all_frames()
        .filter_map(|frame| frame.tag.as_module_frame())
    {
        let codespan_file_id = files.add(&modi.id, &modi.src.text);
        let sf_node_addr = ast::addr_of_sf_node(modi.node.as_ref().unwrap().syntax().clone());
        sf_node_addr_to_file_id.insert(sf_node_addr, codespan_file_id);
    }

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for diagnostic in errors.iter().map(|error| match error {
        LuErr::Parse(ParseErr::Message(msg)) => Diagnostic::error()
            .with_message(msg)
            .with_code("Internal Error"),
        LuErr::Ty(ty_err) => ty_err_to_diagnostic(ty_err, &sf_node_addr_to_file_id),
        LuErr::FS(fs_err) => Diagnostic::error()
            .with_message(fs_err.to_string())
            .with_code("File-IO error"),
        LuErr::Eval(eval_err) => eval_err_to_diagnostic(eval_err, &sf_node_addr_to_file_id),
        LuErr::Ast(ast_err) => ast_err_to_diagnostic(ast_err, &sf_node_addr_to_file_id),
        LuErr::Internal(msg) => Diagnostic::error()
            .with_message(msg)
            .with_code("Internal Error"),
    }) {
        term::emit(&mut writer.lock(), &config, &files, &diagnostic).map_err(|e| e.to_string())?
    }

    Ok(())
}

fn f_id_of_item(item: &SourceCodeItem, sf_node_addr_to_file_id: &HashMap<usize, usize>) -> usize {
    sf_node_addr_to_file_id
        .get(&item.sf_node_addr)
        .expect(&format!("File not present. {}", item.sf_node_addr))
        .clone()
}
