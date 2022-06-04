use std::rc::Rc;

use lu_interpreter_structs::{Function, Signature, Variable};
use lu_syntax::ast::ClosureExprNode;
use rusttyc::TcKey;

use crate::{ScopeFrameTag, TcFunc, TyCheckState};

use super::{fn_cls_ty_check::insert_arguments_into_scope, TypeCheck};

impl TypeCheck for ClosureExprNode {
    fn do_typecheck(
        &self,
        _: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        // TODO is this needed
        let name = Function::closure_name_from_node(&self.clone().into());

        let sign = Signature::from_sign_and_stmt(self.signature(), self.decl_item());
        let required_flags = sign.req_flags();

        let tc_func = TcFunc::from_signature(&sign, ty_state);

        let cls_frame = ScopeFrameTag::TyCFnFrame(name.clone(), required_flags.clone());
        // Closures capture variables in their environment.
        let all_vars_inside_fn = ty_state
            .scope
            .all_vars_inside_fn()
            .cloned()
            .collect::<Vec<_>>();
        let (_, frame) = ty_state.scope.push_frame(cls_frame.clone());
        for v in all_vars_inside_fn {
            frame.insert_var(v);
        }

        // Insert closure, so that ret_stmt can find it... Maybe change content of TyCFnFrame?
        let func = Function::closure_from_node(self.clone().into());
        frame.insert_var(Variable::new_func(Rc::new(func)));

        let self_key = tc_func.self_key.clone();
        insert_arguments_into_scope(tc_func, &sign, ty_state);

        if let Some(fn_block) = self.block_stmt() {
            fn_block.typecheck(ty_state);
        }

        ty_state.scope.pop_frame(&cls_frame);

        Some(self_key)
    }
}
