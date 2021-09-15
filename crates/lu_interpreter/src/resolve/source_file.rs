use lu_syntax::ast::SourceFileNode;
use lu_syntax_elements::BlockType;

use crate::visit_arg::VisitArg;

use super::{Resolve, ResolveArg, Resolver};

impl Resolve for SourceFileNode {
    fn do_resolve_dependant_names(&self, _: &[super::ResolveArg], resolver: &mut Resolver) {
        let stmts = self.statements().unwrap();
        stmts.resolve_dependant_names_with_args(
            &[ResolveArg::Arg(VisitArg::BlockTypeArg(
                BlockType::SourceFileBlock,
            ))],
            resolver,
        );
    }
}
