use crate::evaluate::eval_prelude::*;
use lu_error::lu_source_code_item;
use lu_syntax::ast::{PipeOrValueExprElement, PipedCmdsStmtNode};

impl Evaluable for PipedCmdsStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let (mut prev_val, mut prev_val_decl) = (Value::Nil, lu_source_code_item!()); // The first cmd does not have input
        for cmd in self.piped_args() {
            prev_val = cmd.evaluate_with_args(
                &[EvalArg::CmdInVal {
                    val: prev_val,
                    decl: prev_val_decl,
                }],
                scope,
            )?;
            prev_val_decl = cmd.to_item().into();
        }

        Ok(prev_val)
    }
}

impl Evaluable for PipeOrValueExprElement {
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            PipeOrValueExprElement::PipedCmdsStmt(n) => n.evaluate_with_args(args, scope),
            PipeOrValueExprElement::CmdStmt(n) => n.evaluate_with_args(args, scope),
            PipeOrValueExprElement::ValueExpr(n) => n.evaluate_with_args(args, scope),
        }
    }
}
