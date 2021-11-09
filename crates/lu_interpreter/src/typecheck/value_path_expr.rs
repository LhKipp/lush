use lu_error::TyErr;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{ast::ValuePathExprNode, AstNode};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg, ValueType};

impl TypeCheck for ValuePathExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], state: &mut TyCheckState) -> Option<TcKey> {
        let var_parts = self.var_name_parts_with_item();
        let mut prev_key: Option<TcKey> = None;
        for (part, item) in var_parts {
            if let Some(last_key) = prev_key {
                // we have a part which is dependend on the previous value_path part
                // e.G. $a.b (with part == "b" and last_key = key_of(a))
                if let Some(strct) = state.expect_strct_from_key(&last_key).cloned() {
                    if let Some(key) = strct
                        .field_keys
                        .iter()
                        .find(|field| &field.name == &part)
                        .map(|field| field.ty.clone())
                    {
                        prev_key = Some(key)
                    } else {
                        state.push_err(
                            TyErr::StructDoesNotHaveField {
                                field_name: part.to_string(),
                                strct_decl: state.get_item_of(&strct.self_key).clone(),
                                usage: item,
                            }
                            .into(),
                        );
                        // Immediate return as this can't recover here
                        return Some(
                            state.new_term_key_concretiziesd(self.to_item(), ValueType::Error),
                        );
                    }
                }
            } else {
                // No previous part, this is no field indexing
                if let Some(var_key) = state.expect_key_from_var(&part, item) {
                    prev_key = Some(var_key)
                } else {
                    // Var not present, error key
                    return Some(
                        state.new_term_key_concretiziesd(self.to_item(), ValueType::Error),
                    );
                }
            }
        }

        prev_key
    }
}
