use lu_interpreter_structs::ScopeFrameTag;
use lu_syntax::ast::IfStmtNode;
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg};

impl TypeCheck for IfStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        ty_state.scope.push_frame(ScopeFrameTag::IfStmtFrame);

        if let Some(if_cond) = self.if_condition() {
            if_cond.typecheck(ty_state);
        }

        if let Some(if_block) = self.if_block() {
            if_block.typecheck(ty_state);
        }

        for (elif_cond, elif_block) in self.elif_blocks() {
            if let Some(elif_cond) = elif_cond {
                elif_cond.typecheck(ty_state);
            }
            if let Some(elif_block) = elif_block {
                elif_block.typecheck(ty_state);
            }
        }

        if let Some(else_block) = self.else_block() {
            else_block.typecheck(ty_state);
        }

        ty_state.scope.pop_frame(&ScopeFrameTag::IfStmtFrame);

        None // If does not return
    }
}
