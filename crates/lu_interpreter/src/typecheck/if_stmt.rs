use lu_error::lu_source_code_item;
use lu_interpreter_structs::{ScopeFrameTag, ValueType, Variable};
use lu_syntax::{
    ast::{
        BlockStmtNode, ElseStmtNode, IfElifElseStmtNode, IfElifElseStmtPartElement, IfElifStmtNode,
        IfOptElifOptStmtNode,
    },
    AstToken,
};
use rusttyc::TcKey;

use crate::{visit_arg::VisitArg, TypeCheck, TypeCheckArg};

impl TypeCheck for IfElifElseStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        ty_state.scope.push_frame(ScopeFrameTag::IfStmtFrame);
        for part in self.parts() {
            part.typecheck(ty_state);
        }
        ty_state.scope.pop_frame(&ScopeFrameTag::IfStmtFrame);
        None // If does not return
    }
}

impl TypeCheck for IfElifElseStmtPartElement {
    fn do_typecheck(
        &self,
        _: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        match self {
            IfElifElseStmtPartElement::IfOptElifOptStmt(n) => n.typecheck(ty_state),
            IfElifElseStmtPartElement::IfElifStmt(n) => n.typecheck(ty_state),
            IfElifElseStmtPartElement::ElseStmt(n) => n.typecheck(ty_state),
        }
    }
}

impl TypeCheck for IfElifStmtNode {
    fn do_typecheck(
        &self,
        _: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        if let Some(condition) = self.condition() {
            condition.typecheck(ty_state);
        }
        typecheck_block(self.block(), ty_state);
        None
    }
}

impl TypeCheck for ElseStmtNode {
    fn do_typecheck(
        &self,
        _: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        typecheck_block(self.block(), ty_state);
        None
    }
}

impl TypeCheck for IfOptElifOptStmtNode {
    fn do_typecheck(
        &self,
        _: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        if let Some(opt_expr) = self.rhs_opt() {
            let expr_key = opt_expr
                .typecheck(ty_state)
                .expect("ValueExprElement always returns key");
            ty_state.concretizes_key(
                expr_key,
                ValueType::new_optional(ValueType::Unspecified, lu_source_code_item!()),
            );
        }

        if let Some(var_name) = self.var_name() {
            ty_state.insert_var(Variable::new_nil(
                var_name.to_string(),
                var_name.to_item().into(),
            ));
        }

        typecheck_block(self.block(), ty_state);

        None
    }
}

fn typecheck_block(block: Option<BlockStmtNode>, ty_state: &mut crate::TyCheckState) {
    if let Some(block) = block {
        block.typecheck_with_args(
            &[TypeCheckArg::Arg(VisitArg::BlockTypeArg(
                ScopeFrameTag::IfStmtFrame,
            ))],
            ty_state,
        );
    }
}
