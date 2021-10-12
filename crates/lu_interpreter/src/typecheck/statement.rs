use log::warn;
use lu_syntax::ast::StatementElement;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg};

impl TypeCheck for StatementElement {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        match self {
            // StatementElement::ForStmt(n) => n.typecheck(ty_state),
            // StatementElement::IfStmt(n) => n.typecheck(ty_state),
            // StatementElement::PipedCmdsStmt(n) => n.typecheck(ty_state),
            StatementElement::LetStmt(n) => n.typecheck(ty_state),
            StatementElement::FnStmt(n) => n.typecheck(ty_state),
            StatementElement::CmdStmt(n) => n.typecheck(ty_state),
            StatementElement::RetStmt(n) => n.typecheck(ty_state),
            StatementElement::ValueExpr(n) => n.typecheck(ty_state),
            _ => {
                warn!("NOT TY CHECKING whole StatementElement!!! REturning NONE");
                None
            }
        }
    }
}
