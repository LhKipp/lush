use std::{
    convert::TryInto,
    sync::{Arc, Weak},
};

use log::debug;
use lu_error::{util::Outcome, SourceCodeItem, TyErr};
use lu_interpreter_structs::{Strct, StrctField, ValueType, Variable};
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{CmdArgElement, CmdStmtNode, ValueExprElement},
    AstNode, AstToken,
};
use parking_lot::RwLock;
use rusttyc::TcKey;

use crate::TypeCheckArg;

pub(crate) fn do_extra_ty_check_select_cmd(
    cmd_stmt: &CmdStmtNode,
    args: &[TypeCheckArg],
    ty_state: &mut crate::TyCheckState,
) -> Option<TcKey> {
    debug!("Ty checking a select stmt {}", cmd_stmt);
    assert_eq!(cmd_stmt.get_cmd_name(), "select");

    let in_piped_arg_key = args
        .iter()
        .find_map(|arg| arg.as_cmd_stmt())
        .expect("CmdStmt arg always passed");

    // First we need to make sure we are being passed a table and we need to get the inner
    // ty
    if let Some(passed_ty) = ty_state.expect_ty_of_key(in_piped_arg_key.clone()) {
        if let ValueType::Array { inner_ty, .. } = passed_ty {
            if let ValueType::Strct(strct_decl) = &*inner_ty {
                let strct_decl = Weak::upgrade(strct_decl).unwrap();
                let l_strct_decl = strct_decl.read();

                // Parse args
                let args = get_select_args(cmd_stmt);
                let args = ty_state.ok_and_record(args);

                // Generate new strct decl according to columns
                let fields = get_selected_fields(args, &l_strct_decl);
                let fields = ty_state.ok_and_record(fields);
                let decl = cmd_stmt.to_item();
                let name = new_default_strct_name(&decl);

                let strct = Arc::new(RwLock::new(Strct::new(name.clone(), fields, decl)));
                // Insert strct
                let cur_mod_frame = ty_state
                    .scope
                    .get_cur_mod_frame()
                    .expect("Select only found below module frames");
                cur_mod_frame.insert_var(Variable::new_strct_decl_arc(strct.clone()));

                // Return correct ret_ty for select
                let ret_key = ty_state.new_term_key_concretiziesd(
                    cmd_stmt.to_item(),
                    ValueType::new_array(
                        ValueType::Strct(Arc::downgrade(&strct)),
                        cmd_stmt.to_item(),
                    ),
                );
                return Some(ret_key);
            } else {
                ty_state.push_err(
                    TyErr::ExpectedStmtToReturnAnArrayOfStrcts {
                        stmt_with_wrong_ret: ty_state.get_item_of(in_piped_arg_key).clone(),
                        found_ty: format!("[{}]", inner_ty.to_string()),
                    }
                    .into(),
                )
            }
        } else {
            // in is not array. Already handled by tycheck cmd
        }
    }

    None
}

pub(crate) fn new_default_strct_name(cmd_stmt_decl: &SourceCodeItem) -> String {
    format!("Select_textrange_{}", cmd_stmt_decl.display_range())
}

struct SelectArgs {
    columns: Vec<(String, SourceCodeItem)>,
}

fn get_select_args(cmd_stmt: &CmdStmtNode) -> Outcome<SelectArgs> {
    let mut arg_iter = cmd_stmt.args();
    let mut args = SelectArgs { columns: vec![] };
    let mut errs = vec![];
    while let Some(arg) = arg_iter.next() {
        if let CmdArgElement::ValueExpr(ValueExprElement::BareWord(bw)) = arg {
            args.columns.push((bw.text_trimmed(), bw.to_item()))
        } else if let CmdArgElement::ValueExpr(ValueExprElement::StringExpr(string)) = arg {
            args.columns.push((string.text_trimmed(), string.to_item()))
        } else if let CmdArgElement::ValueExpr(ValueExprElement::ValuePathExpr(var)) = arg {
            errs.push(TyErr::SelectArgMustBeBareWordOrString { arg: var.to_item() }.into())
        }
    }

    Outcome::new(args, errs)
}

fn get_selected_fields(args: SelectArgs, strct_decl: &Strct) -> Outcome<Vec<StrctField>> {
    args.columns
        .into_iter()
        .enumerate()
        .map(|(col_num, (col_name, col_arg_item))| {
            if let Some(field) = strct_decl.fields.iter().find_map(|field| {
                if field.name == col_name {
                    Some(field)
                } else {
                    None
                }
            }) {
                Ok(StrctField::new(
                    field.name.clone(),
                    field.ty.clone(),
                    col_num.try_into().unwrap(),
                    col_arg_item,
                ))
            } else {
                Err(TyErr::StructDoesNotHaveField {
                    field_name: col_name,
                    strct_decl: strct_decl.decl.clone(),
                    usage: col_arg_item,
                }
                .into())
            }
        })
        .collect()
}
