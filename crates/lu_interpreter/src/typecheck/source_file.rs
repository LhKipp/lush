use lu_syntax::ast::SourceFileNode;
use rusttyc::TcKey;

use crate::{visit_arg::VisitArg, TypeCheckArg};

use super::{TypeCheck, TypeChecker};

impl TypeCheck for SourceFileNode {
    fn do_typecheck(
        &self,
        args: &[super::TypeCheckArg],
        ty_state: &mut TypeChecker,
    ) -> Option<TcKey> {
        let f_path = match args.get(0) {
            Some(TypeCheckArg::Arg(VisitArg::SourceFilePath(f_path))) => f_path,
            _ => unreachable!("Passing of file name is mandatory"),
        };

        let stmts = self.statements().unwrap();
        stmts.typecheck_with_args(
            &[TypeCheckArg::Arg(VisitArg::SourceFileBlock(f_path.clone()))],
            ty_state,
        )
    }
}
