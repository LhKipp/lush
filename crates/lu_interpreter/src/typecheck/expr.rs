use itertools::{EitherOrBoth, Itertools};
use log::warn;
use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, CmdOrValueExprElement, NumberExprNode, StrctCtorExprNode,
        StringExprNode, TableExprNode, ValueExprElement,
    },
    AstNode,
};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg, ValueType};

impl TypeCheck for ValueExprElement {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self {
            ValueExprElement::BareWord(n) => n.typecheck(state),
            ValueExprElement::NumberExpr(n) => n.typecheck(state),
            ValueExprElement::MathExpr(_) => todo!(),
            ValueExprElement::StringExpr(n) => n.typecheck(state),
            ValueExprElement::ValuePathExpr(n) => n.typecheck(state),
            ValueExprElement::ArrayExpr(n) => n.typecheck(state),
            ValueExprElement::TableExpr(n) => n.typecheck(state),
            ValueExprElement::StrctCtorExpr(n) => n.typecheck(state),
        }
    }
}

impl TypeCheck for CmdOrValueExprElement {
    fn do_typecheck(&self, args: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self {
            CmdOrValueExprElement::CmdStmt(n) => n.typecheck_with_args(args, state),
            CmdOrValueExprElement::PipedCmdsStmt(n) => n.typecheck_with_args(args, state),
            CmdOrValueExprElement::ValueExpr(n) => n.typecheck_with_args(args, state),
        }
    }
}

impl TypeCheck for BareWordToken {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        let key = ty_state.checker.new_term_key();
        ty_state
            .checker
            .impose(key.concretizes_explicit(ValueType::BareWord))
            .unwrap();
        Some(key)
    }
}

impl TypeCheck for NumberExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        let key = ty_state.checker.new_term_key();
        ty_state
            .checker
            .impose(key.concretizes_explicit(ValueType::Number))
            .unwrap();
        Some(key)
    }
}

impl TypeCheck for StringExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        let key = ty_state.new_term_key(self.to_item());
        ty_state
            .checker
            .impose(key.concretizes_explicit(ValueType::String))
            .unwrap();
        Some(key)
    }
}

impl TypeCheck for ArrayExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], _state: &mut TyCheckState) -> Option<TcKey> {
        todo!("TODO find sub type")
        // let key = ty_state.checker.new_term_key();
        // ty_state
        //     .checker
        //     .impose(key.concretizes_explicit(ValueType::ArrayExprNode))
        //     .unwrap();
        // Some(key)
    }
}

impl TypeCheck for TableExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], _state: &mut TyCheckState) -> Option<TcKey> {
        todo!()
    }
}

impl TypeCheck for StrctCtorExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        // panic!("Ty chekcing ");
        let strct_key = ty_state.expect_strct(&self.name(), self.to_item());
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
                .merge_join_by(usages, |decl, field| Ord::cmp(&decl.0, &field.0))
            {
                match either_or {
                    EitherOrBoth::Both((_, decl_key), (_, usage_key)) => {
                        ty_state.equate_keys(decl_key.clone(), usage_key);
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
