use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, CmdOrValueExprElement, NumberExprNode, StringExprNode,
        TableExprNode, ValueExprElement, ValuePathExprNode,
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

impl TypeCheck for ValuePathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], _state: &mut TyCheckState) -> Option<TcKey> {
        todo!("Find other var and return that key")
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
