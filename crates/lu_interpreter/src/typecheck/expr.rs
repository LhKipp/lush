use itertools::{EitherOrBoth, Itertools};
use log::{debug, warn};
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, BooleanExprNode, FileNameElement, NumberExprNode,
        OptionalExprNode, StrctCtorExprNode, StringExprNode, ValueExprElement,
    },
    AstElement, AstNode, AstToken,
};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg, ValueType};

impl TypeCheck for ValueExprElement {
    fn do_typecheck(&self, args: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self {
            ValueExprElement::BooleanExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::BareWord(n) => n.typecheck_with_args(args, state),
            ValueExprElement::NumberExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::MathExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::StringExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::ValuePathExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::ArrayExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::TableExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::StrctCtorExpr(n) => n.typecheck_with_args(args, state),
            ValueExprElement::CmdStmt(n) => n.typecheck_with_args(args, state),
            ValueExprElement::FileName(n) => n.typecheck_with_args(args, state),
            ValueExprElement::OptionalExpr(n) => n.typecheck_with_args(args, state),
        }
    }
}

impl TypeCheck for OptionalExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        match self.value() {
            Some(inner) => {
                let result = ty_state.new_term_key_concretiziesd(
                    self.to_item(),
                    ValueType::new_optional(ValueType::Unspecified, inner.to_item()),
                );

                if let Some(inner_key) = inner.typecheck(ty_state) {
                    let inner_opt_key = ty_state
                        .expect_opt_inner_ty_from_key(result)
                        .expect("Prev inserted, always present");
                    ty_state.equate_keys(inner_opt_key, inner_key);
                }

                Some(result)
            }
            None => Some(ty_state.new_term_key_concretiziesd(
                self.to_item(),
                ValueType::new_optional(ValueType::Unspecified, self.to_item()),
            )),
        }
    }
}

impl TypeCheck for BareWordToken {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        Some(ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::BareWord))
    }
}

impl TypeCheck for FileNameElement {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        if let Some(err) = self.validate() {
            ty_state.push_err(err);
        }
        Some(ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::FileName))
    }
}

impl TypeCheck for NumberExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        Some(ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::Number))
    }
}

impl TypeCheck for StringExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        Some(ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::String))
    }
}

impl TypeCheck for ArrayExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        // Equate all inner ty's (they must be of the same ty)
        let mut prev_inner_tc: Option<TcKey> = None;
        for array_elem in self.values() {
            let elem_key = array_elem
                .typecheck(state)
                .expect("Array inner elems always have type");
            if let Some(prev_inner_tc) = prev_inner_tc {
                state.equate_keys(prev_inner_tc.clone(), elem_key.clone());
            }
            prev_inner_tc = Some(elem_key);
        }
        let array_key = state.new_term_key_concretiziesd(
            self.to_item(),
            ValueType::new_array(ValueType::Unspecified, self.to_item()),
        );

        if let Some(prev_inner_tc) = prev_inner_tc {
            // If we have found an prev_inner_tc, we can specify the arr inner ty more concretly
            let inner_ty_key = state
                .expect_arr_inner_ty_from_key(array_key)
                .expect("Prev inserted, always present");
            state.equate_keys(prev_inner_tc, inner_ty_key);
        }

        Some(array_key)
    }
}

impl TypeCheck for StrctCtorExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        // panic!("Ty chekcing ");
        let strct_key = ty_state.expect_strct_from_usage(&self.name(), self.to_item());
        if let Some(strct_key) = strct_key.cloned() {
            let usage_key = ty_state.new_term_key_equated(self.to_item(), strct_key.self_key);
            // Check that all fields have correct ty
            let usages = self
                .fields()
                .map(|field_ctor| {
                    (
                        field_ctor.field_name(),
                        field_ctor
                            .value()
                            .expect("TODO")
                            .typecheck(ty_state)
                            .unwrap(),
                    )
                })
                .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                .collect::<Vec<_>>();
            for either_or in strct_key
                .field_keys
                .iter()
                .merge_join_by(usages, |decl, field| Ord::cmp(&decl.name, &field.0))
            {
                match either_or {
                    EitherOrBoth::Both(field, (_, usage_key)) => {
                        debug!("Equating key: {:?} with field {:?}", usage_key, field);
                        ty_state.equate_keys(field.ty.clone(), usage_key);
                    }
                    EitherOrBoth::Left(_) => todo!("Generate error for missing key."),
                    EitherOrBoth::Right(_) => {
                        todo!("Generate error for provided but not decl key")
                    }
                }
            }

            Some(usage_key)
        } else {
            warn!("StructCtor with name {} not found", self.name());
            None
        }
    }
}
impl TypeCheck for BooleanExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        Some(ty_state.new_term_key_concretiziesd(self.to_item(), ValueType::Bool))
    }
}
