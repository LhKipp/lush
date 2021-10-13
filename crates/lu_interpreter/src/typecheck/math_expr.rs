use lu_syntax::ast::{MathExprNode, OperatorExprElement, ValueExprElement};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg};

impl TypeCheck for MathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self.operator() {
            OperatorExprElement::PlusSign(_) => {
                let (lhs, _) = equate(&self.lhs(), &self.rhs(), state);
                Some(lhs)
            }
            OperatorExprElement::MinusSign(_) => todo!(),
            OperatorExprElement::MultSign(_) => todo!(),
            OperatorExprElement::DivSign(_) => todo!(),
            OperatorExprElement::LessThanSign(_) => todo!(),
            OperatorExprElement::LessOrEqualSign(_) => todo!(),
            OperatorExprElement::EqualitySign(_) => todo!(),
            OperatorExprElement::InequalitySign(_) => todo!(),
            OperatorExprElement::BiggerThanSign(_) => todo!(),
            OperatorExprElement::BiggerOrEqualSign(_) => todo!(),
            OperatorExprElement::RightStream(_) => todo!(),
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
