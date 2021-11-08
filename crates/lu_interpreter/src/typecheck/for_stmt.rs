use lu_interpreter_structs::{ScopeFrameTag, Value, VarDeclNode, Variable};
use lu_syntax::{ast::ForStmtNode, AstToken};
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg};

impl TypeCheck for ForStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        ty_state.scope.push_frame(ScopeFrameTag::ForStmtFrame);

        let var_names: Vec<String> = self
            .var_names()
            .into_iter()
            .map(|decl| decl.text_trimmed())
            .collect();
        assert!(var_names.len() > 0, "TODO");

        ty_state.insert_var(Variable::new(
            var_names[0].clone(),
            Value::Nil,
            VarDeclNode::ForStmt(self.clone(), 0),
        ));

        // TODO check that iterated_value is either array or string. Currently thats not possible
        let _ = self.iterated_value();

        if let Some(block) = self.block() {
            block.typecheck(ty_state);
        }

        ty_state.scope.pop_frame(&ScopeFrameTag::ForStmtFrame);

        None // For does not return
    }
}
