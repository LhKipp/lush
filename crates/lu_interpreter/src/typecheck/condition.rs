use lu_syntax::ast::ConditionElement;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg, ValueType};

impl TypeCheck for ConditionElement {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        let ret_key = match self {
            ConditionElement::PipedCmdsStmt(cmd) => cmd.typecheck(ty_state),
            ConditionElement::ValueExpr(expr) => expr.typecheck(ty_state),
            ConditionElement::CmdStmt(cmd) => cmd.typecheck(ty_state),
        };
        if let Some(ret_key) = ret_key {
            ty_state.concretizes_key(ret_key, ValueType::Bool);
        } else {
            unreachable!("Cmd or ValueExpr always returning a key")
        }

        ret_key
    }
}
