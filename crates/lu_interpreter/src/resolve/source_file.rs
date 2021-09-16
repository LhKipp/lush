use lu_syntax::ast::SourceFileNode;
use lu_syntax_elements::BlockType;

use crate::visit_arg::VisitArg;

use super::{Resolve, ResolveArg, Resolver};

impl Resolve for SourceFileNode {
    fn do_resolve_dependant_names(&self, args: &[super::ResolveArg], resolver: &mut Resolver) {
        let f_path = match args.get(0) {
            Some(ResolveArg::Arg(VisitArg::SourceFilePath(f_path))) => f_path,
            _ => unreachable!("Passing of file name is mandatory"),
        };

        let stmts = self.statements().unwrap();

        stmts.resolve_dependant_names_with_args(
            &[ResolveArg::Arg(VisitArg::SourceFileBlock(f_path.clone()))],
            resolver,
        );
    }
}
