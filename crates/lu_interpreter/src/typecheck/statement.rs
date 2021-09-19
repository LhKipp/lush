use lu_syntax::ast::StatementElement;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg};

impl TypeCheck for StatementElement {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TypeChecker,
    ) -> Option<TcKey> {
        match self {
            // StatementElement::ForStmt(n) => n.typecheck(ty_state),
            StatementElement::LetStmt(n) => n.typecheck(ty_state),
            // StatementElement::FnStmt(n) => n.typecheck(ty_state),
            // StatementElement::IfStmt(n) => n.typecheck(ty_state),
            StatementElement::CmdStmt(n) => n.typecheck(ty_state),
            // StatementElement::PipedCmdsStmt(n) => n.typecheck(ty_state),
            _ => None,
        }
    }
}
