use lu_interpreter_structs::Signature;
use lu_syntax::ast::FnStmtNode;
use rusttyc::TcKey;

use crate::{ScopeFrameTag, TyCheckState};

use super::{fn_cls_ty_check::insert_arguments_into_scope, TypeCheck};

impl TypeCheck for FnStmtNode {
    fn do_typecheck(
        &self,
        _: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        let fn_name = if let Some(name) = self.name() {
            name
        } else {
            return None;
        };

        let sign = Signature::from_sign_and_stmt(self.signature(), self.decl_item());
        let req_flags = sign.req_flags();

        let tc_func = ty_state
            .expect_tc_cmd_from_cmd_usage(&fn_name, &req_flags, self.decl_item())
            .expect("Always works");

        let fn_frame = ScopeFrameTag::TyCFnFrame(fn_name.clone(), req_flags);
        ty_state.scope.push_frame(fn_frame.clone());

        insert_arguments_into_scope(tc_func, &sign, ty_state);

        if let Some(fn_block) = self.block_stmt() {
            fn_block.typecheck(ty_state);
        }

        ty_state.scope.pop_frame(&fn_frame);

        // A fn stmt doesn't return a value
        None
    }
}
