use lu_syntax::ast::SourceFileNode;
use rusttyc::TcKey;

use super::{TyCheckState, TypeCheck};

impl TypeCheck for SourceFileNode {
    fn do_typecheck(
        &self,
        _: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        self.block().unwrap().typecheck(ty_state)
    }
}
