use lu_interpreter_structs::ValueType;
use lu_syntax::{
    ast::{MathExprNode, OperatorExprElement, ValueExprElement},
    AstNode,
};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg};

impl TypeCheck for MathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self.operator() {
            OperatorExprElement::PlusSign(_)
            | OperatorExprElement::MinusSign(_)
            | OperatorExprElement::MultSign(_)
            | OperatorExprElement::DivSign(_) => {
                let (lhs, _) = equate(&self.lhs(), &self.rhs(), state);
                Some(lhs)
            }
            OperatorExprElement::LessThanSign(_)
            | OperatorExprElement::LessOrEqualSign(_)
            | OperatorExprElement::BiggerThanSign(_)
            | OperatorExprElement::BiggerOrEqualSign(_)
            | OperatorExprElement::InequalitySign(_)
            | OperatorExprElement::EqualitySign(_) => {
                equate(&self.lhs(), &self.rhs(), state);
                Some(state.new_term_key_concretiziesd(self.to_item(), ValueType::Bool))
            }
            OperatorExprElement::DivAssignSign(_) => todo!(),
            OperatorExprElement::MulAssignSign(_) => todo!(),
            OperatorExprElement::AddAssignSign(_) => todo!(),
            OperatorExprElement::MinAssignSign(_) => todo!(),
            OperatorExprElement::AssignSign(_) => {
                equate(&self.lhs(), &self.rhs(), state);
                // Assignment does not return type
                None
            }
        }
    }
}

/// Equates lhs with rhs and returns (LhsKey, RhsKey)
fn equate(
    lhs: &ValueExprElement,
    rhs: &ValueExprElement,
    ty_state: &mut TyCheckState,
) -> (TcKey, TcKey) {
    let lhs_key = lhs.typecheck(ty_state).unwrap();
    let rhs_key = rhs.typecheck(ty_state).unwrap();
    ty_state.equate_keys(lhs_key, rhs_key);
    (lhs_key, rhs_key)
}
