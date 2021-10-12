use log::debug;
use lu_syntax::ast::{MathExprNode, OperatorExprElement};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg};

impl TypeCheck for MathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self.operator() {
            OperatorExprElement::PlusSign(_) => todo!(),
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
                debug!("TyChecking assignment");
                let lhs = self.lhs().unwrap();
                let rhs = self.rhs().unwrap();

                let lhs_key = lhs.typecheck(state).unwrap();
                let rhs_key = rhs.typecheck(state).unwrap();
                state.equate_keys(lhs_key, rhs_key);
                // Assignment does not return type
                None
            }
        }
    }
}
