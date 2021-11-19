use crate::{evaluate::eval_prelude::*, handle_dbg_intervention_before};
use lu_syntax::ast::{
    ElseStmtNode, HasAstId, IfElifElseStmtNode, IfElifElseStmtPartElement, IfElifStmtNode,
    IfOptElifOptStmtNode,
};

impl Evaluable for IfElifElseStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        for part in self.parts() {
            match part.evaluate(scope)? {
                Value::Bool(has_evaluated) => {
                    if has_evaluated {
                        return Ok(Value::Nil);
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(Value::Nil)
    }
}

// Parts returns whether their block have been evaluated
impl Evaluable for IfElifElseStmtPartElement {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        match self {
            IfElifElseStmtPartElement::IfOptElifOptStmt(n) => n.evaluate(scope),
            IfElifElseStmtPartElement::IfElifStmt(n) => n.evaluate(scope),
            IfElifElseStmtPartElement::ElseStmt(n) => n.evaluate(scope),
        }
    }
}

impl Evaluable for ElseStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        if let Some(block) = self.block() {
            block.evaluate(scope)?;
        };
        Ok(true.into())
    }
}

impl Evaluable for IfElifStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let cond_val = self.condition().unwrap().evaluate(scope)?;
        let cond_val = match cond_val.coerce_to_bool() {
            None => {
                // TODO shouldnt happen this should be cahtche by ty
                //  check wheter it isn't
                return Err(LuErr::Eval(EvalErr::NotConvertibleToBool(
                    self.condition().unwrap().to_item(),
                ))
                .into());
            }
            Some(v) => v,
        };

        if cond_val {
            if let Some(block) = self.block() {
                block.evaluate(scope)?;
            }
            Ok(true.into())
        } else {
            Ok(false.into())
        }
    }
}

impl Evaluable for IfOptElifOptStmtNode {
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let optional_val = self.rhs_opt().unwrap().evaluate(scope)?;
        let (_, cond_val) = optional_val.as_optional().expect("Must be optional");

        let dbg_result = lu_dbg::before_eval(
            &format!("{}", self.fmt_for_debug()),
            self.get_ast_id(),
            scope,
        )?;
        handle_dbg_intervention_before!(dbg_result, scope);

        if let Some(cond_val) = cond_val.clone() {
            let var_name = self.var_name().unwrap();
            scope
                .lock()
                .push_frame(ScopeFrameTag::IfStmtFrame)
                .1
                .insert_var(Variable::new(
                    var_name.to_string(),
                    *cond_val,
                    var_name.to_item().into(),
                ));
            let result = self
                .block()
                .unwrap()
                .evaluate_with_args(&[EvalArg::BlockNoPushFrame], scope);
            scope.lock().pop_frame(&ScopeFrameTag::IfStmtFrame);
            result?;

            Ok(true.into())
        } else {
            Ok(false.into())
        }
    }
}
