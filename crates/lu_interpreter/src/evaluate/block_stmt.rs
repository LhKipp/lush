#![allow(unused_imports)]
use crate::evaluate::eval_prelude::*;
use contracts::ensures;
use lu_syntax::{
    ast::{BlockStmtNode, ConditionElement, IfBlockNode},
    ast::{IfStmtNode, StatementElement, StrctStmtNode},
};

impl Evaluable for BlockStmtNode {
    fn do_evaluate(&self, args: &[EvalArg], scope: &mut Arc<Mutex<Scope<Variable>>>) -> EvalResult {
        let should_push_frame = !args.contains(&EvalArg::BlockNoPushFrame);

        if should_push_frame {
            scope.lock().push_frame(ScopeFrameTag::BlockFrame);
        }

        let mut result = Value::Nil;

        for stmt in self
            .statements()
            .filter(|stmt| !matches!(stmt, StatementElement::FnStmt(_)))
        {
            match stmt.evaluate(scope) {
                Ok(v) => result = v,
                Err(e) => {
                    if should_push_frame {
                        scope.lock().pop_frame(&ScopeFrameTag::BlockFrame);
                    }
                    return Err(e);
                }
            }
        }
        if should_push_frame {
            scope.lock().pop_frame(&ScopeFrameTag::BlockFrame);
        }
        Ok(result)
    }
}
