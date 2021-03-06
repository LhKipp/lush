use lu_interpreter_structs::ValueType;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{
    ast::{MathExprNode, OperatorExprElement, ValueExprElement},
    AstNode,
};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg};

impl TypeCheck for MathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        match self.operator() {
            OperatorExprElement::AsKeyword(_) => {
                // let lhs_key = self.lhs().typecheck(state).expect("ValueExpr always gives key");
                // TODO this is currently a noop, as anything can be Any. It should somehow be
                // expressable, that only any is allowed here
                // state.concretizes_key(lhs_key, ValueType::Any);

                if let Some(ty) = self.rhs_as_lu_type() {
                    match ValueType::from_node_or_err_resolve_strct_name(&ty, &state.scope)
                        .as_results()
                    {
                        Ok(ty) => {
                            return Some(state.new_term_key_concretiziesd(self.to_item(), ty))
                        }
                        Err(e) => state.push_errs(e),
                    }
                } else {
                    // Either incomplete input, or grammar already gave warning here :)
                }
                None
            }
            OperatorExprElement::PlusSign(_)
            | OperatorExprElement::MinusSign(_)
            | OperatorExprElement::MultSign(_)
            | OperatorExprElement::DivSign(_) => {
                let (lhs, _) = equate(&self.lhs(), &self.rhs(), state);
                Some(state.new_term_key_equated(self.to_item(), lhs))
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
            OperatorExprElement::OrKeyword(_) | OperatorExprElement::AndKeyword(_) => {
                concretize(&self.lhs(), ValueType::Bool, state);
                concretize(&self.rhs(), ValueType::Bool, state);
                Some(state.new_term_key_concretiziesd(self.to_item(), ValueType::Bool))
            }
            OperatorExprElement::DivAssignSign(_)
            | OperatorExprElement::MulAssignSign(_)
            | OperatorExprElement::AddAssignSign(_)
            | OperatorExprElement::MinAssignSign(_) => {
                concretize(&self.lhs(), ValueType::Number, state);
                concretize(&self.rhs(), ValueType::Number, state);
                None
            }
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
fn concretize(expr: &ValueExprElement, ty: ValueType, ty_state: &mut TyCheckState) {
    let key = expr.typecheck(ty_state).unwrap();
    ty_state.concretizes_key(key, ty);
}
