use std::rc::Rc;

use crate::evaluate::eval_prelude::*;
use log::trace;
use lu_syntax::ast::RetStmtNode;

impl Evaluable for RetStmtNode {
    fn dbg_settings(&self) -> &'static [DbgSetting] {
        &[DbgSetting::StopDbgBeforeEval]
    }
    fn do_evaluate(&self, _: &[EvalArg], scope: &mut SyScope) -> EvalResult {
        let mut ret_val = self.returned_val().unwrap().evaluate(scope)?;
        // Closures capture the environment (outer function) on return
        if let Value::Command(cmd) = &mut ret_val {
            if let Some(func) = cmd.as_function() {
                if func.is_closure() {
                    let mut cls_copy = func.clone();
                    let l_scope = scope.lock();
                    cls_copy.captured_vars = l_scope.all_vars_captured_by_closure();
                    trace!(
                        "Returning closure {} with captured vars {:?}",
                        cls_copy.name,
                        cls_copy.captured_vars
                    );
                    return Err(RetValOrErr::RetVal(Value::Command(Rc::new(cls_copy))));
                }
            }
        }
        Err(RetValOrErr::RetVal(ret_val))
    }
}
