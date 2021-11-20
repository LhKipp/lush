use lu_error::{AstErr, LuErr};

use crate::{ast::SourceFileNode, AstNode};

pub fn validate_sign(node: &SourceFileNode) -> Vec<LuErr> {
    let mut errors: Vec<LuErr> = vec![];

    for fn_stmt in node
        .block()
        .statements()
        .filter_map(|statement| statement.as_fn_stmt().cloned())
    {
        if let Some(sign) = fn_stmt.signature() {
            let mut first_opt_var = None;
            for arg in sign.args() {
                if arg.opt_modifier().is_some() {
                    first_opt_var = first_opt_var.or(Some(arg));
                } else if let Some(first_opt_arg) = first_opt_var.as_ref() {
                    // Non optional but optional before
                    errors.push(
                        AstErr::ReqArgAfterOptionalArg {
                            req_arg: arg.to_item(),
                            opt_arg: first_opt_arg.to_item(),
                        }
                        .into(),
                    );
                }
            }
            match (first_opt_var, sign.var_arg()) {
                (Some(first_opt_var), Some(var_arg)) => errors.push(
                    AstErr::VarArgAfterOptionalArg {
                        var_arg: var_arg.to_item(),
                        opt_arg: first_opt_var.to_item(),
                    }
                    .into(),
                ),
                _ => {}
            }
        }
    }
    errors
}
