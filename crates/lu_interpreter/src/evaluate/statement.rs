use crate::evaluate::eval_prelude::*;
use lu_syntax::ast::StatementElement;

impl Evaluable for StatementElement {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            // Statements that do not print their result
            StatementElement::IfStmt(n) => n.evaluate(scope),
            StatementElement::LetStmt(n) => n.evaluate(scope),
            StatementElement::FnStmt(n) => n.evaluate(scope),
            StatementElement::ForStmt(n) => n.evaluate(scope),
            StatementElement::RetStmt(n) => n.evaluate(scope),

            // Statements that do print their result
            _ => {
                let value = match self {
                    StatementElement::CmdStmt(n) => n.evaluate(scope)?,
                    StatementElement::PipedCmdsStmt(n) => n.evaluate(scope)?,
                    StatementElement::ValueExpr(n) => n.evaluate(scope)?,
                    _ => unreachable!(),
                };
                if let Some(silence) = get_silence_stmt_returns(&scope.lock()) {
                    if silence {
                        return Ok(value); // Early return if no printing of statement returns is asked
                    }
                }
                // Nil does not get printed
                if value == Value::Nil {
                    return Ok(value);
                }

                debug!(
                    "Found Stmt/value which gets printed: ({}/{})",
                    self.to_string(),
                    value
                );
                let val_as_str = value.to_string();
                let shortened = match &value {
                    // Because println adds a newline, we have to strip the newline from external
                    // cmds (if they have had one)
                    Value::BareWord(s) => s.strip_suffix("\n").unwrap_or(&val_as_str),
                    _ => &val_as_str,
                };
                println!("{}", shortened);
                Ok(value)
            }
        }
    }
}
