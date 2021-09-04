use lu_syntax::ast::SourceFileNode;

use super::{TypeCheck, TypeChecker};

impl TypeCheck for SourceFileNode {
    fn do_typecheck(&self, _: &[super::TypeCheckArg], ty_state: &mut TypeChecker) {
        let stmts = self.statements().unwrap();
        stmts.typecheck(ty_state);
    }
}
