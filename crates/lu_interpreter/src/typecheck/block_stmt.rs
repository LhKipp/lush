#![allow(unused_imports)]
use contracts::ensures;
use log::debug;
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{BlockStmtNode, IfStmtNode, StatementElement},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use lu_value::Value;
use rusttyc::TcKey;

use crate::{
    visit_arg::VisitArg, EvalArg, Evaluable, Function, Interpreter, ScopeFrameTag, TcEntry, TcFunc,
    TypeCheckArg, TypeChecker, Variable,
};

use super::TypeCheck;

impl TypeCheck for BlockStmtNode {
    fn do_typecheck(
        &self,
        args: &[super::TypeCheckArg],
        ty_checker: &mut TypeChecker,
    ) -> Option<TcKey> {
        let frame_to_pop = match args.get(0) {
            Some(TypeCheckArg::Arg(VisitArg::SourceFileBlock(f_path))) => {
                if let Err(e) = ty_checker.scope.set_cur_source_frame(f_path) {
                    debug!("SourceFileBlock type check error which should not happen");
                    ty_checker.errors.push(e);
                    return None;
                }
                add_entry_for_funcs(ty_checker);

                None
            }
            Some(TypeCheckArg::Arg(VisitArg::BlockTypeArg(b_type))) => {
                let frame_type: ScopeFrameTag = b_type.clone().into();
                ty_checker.scope.push_frame(frame_type.clone());
                Some(frame_type)
            }
            _ => unreachable!("Passing of either arg is required"),
        };

        let mut result = None;
        for stmt in self.statements() {
            result = stmt.typecheck(ty_checker);
        }

        if let Some(to_pop) = frame_to_pop {
            ty_checker.scope.pop_frame(&to_pop);
        }

        result
    }
}

fn add_entry_for_funcs(ty_checker: &mut TypeChecker) {
    // Handle funcs in the source file block first
    let var_funcs_to_insert = ty_checker
        .scope
        .cur_frame()
        .elems
        .iter()
        .filter_map(|(_, var)| {
            var.val_as_callable()
                .map(|callable| callable.as_func())
                .flatten()
                .map(|func| (var.clone(), func.clone()))
        })
        .collect::<Vec<_>>();
    for (var, func) in var_funcs_to_insert {
        let tc_func = TcFunc::from_func(func, ty_checker);
        ty_checker.tc_func_table.insert(var, tc_func);
    }
}
