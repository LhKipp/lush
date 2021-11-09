use crate::{evaluate::eval_prelude::*, handle_dbg_intervention_before};
use lu_syntax::ast::{ForStmtNode, HasAstId};

impl Evaluable for ForStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let block = self.block().unwrap();
        if block.is_empty() {
            debug!("Empty for stmt");
            // Empty for block. Suspicious?. Should maybe be a warning?
        }

        let text_till_block = self.text_till_block();
        let var_names: Vec<String> = self
            .var_names()
            .into_iter()
            .map(|decl| decl.text().to_string())
            .collect();
        assert!(var_names.len() > 0);
        let iterated_val = self.iterated_value().unwrap();

        let result =
            lu_dbg::before_eval(&format!("{}", text_till_block), self.get_ast_id(), scope)?;
        handle_dbg_intervention_before!(result, scope);

        let iterated_val = iterated_val.evaluate(scope)?;
        let vals_to_iterate = if let Some(array) = iterated_val.as_array() {
            assert_eq!(var_names.len(), 1);
            // TODO ret error
            debug!("Iterating over array");
            (**array).clone()
        } else if let Some(str_to_iter) = iterated_val.as_string() {
            assert_eq!(var_names.len(), 1);
            // TODO ret error
            debug!("Iterating over string {}", str_to_iter);
            str_to_iter
                .chars()
                .map(|c| Value::String(c.into()))
                .collect()
        } else {
            // Error
            todo!()
        };

        // We entered the for loop. We need to push a new scope and set the vars
        for (i, val) in vals_to_iterate.into_iter().enumerate() {
            // We have to do before eval, before evaluating the iterated_val once. Therefore
            // the first iteration does not need before_eval
            if i != 0 {
                let result = lu_dbg::before_eval(&text_till_block, self.get_ast_id(), scope)?;
                handle_dbg_intervention_before!(result, scope);
            }
            {
                let var = Variable::new(
                    var_names[0].clone(),
                    val,
                    VarDeclNode::ForStmt(self.clone(), 0),
                );
                scope
                    .lock()
                    .push_frame(ScopeFrameTag::ForStmtFrame)
                    .1
                    .insert_var(var);
            }
            // We pushed ForStmtFrame, block doesn't need to push additional frame
            // Make sure we deallocate ForStmtFrame before returning
            let eval_result = block.evaluate_with_args(&[EvalArg::BlockNoPushFrame], scope);
            {
                let mut scope = scope.lock();
                scope.pop_frame(&ScopeFrameTag::ForStmtFrame);
            }
            eval_result?;

            lu_dbg::after_eval(&self.get_ast_id(), scope);
        }

        Ok(Value::Nil)
    }
}
