use itertools::Itertools;
use log::warn;
use lu_error::TyErr;
use lu_interpreter_structs::ValueType;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{ast::TableExprNode, AstNode, AstToken};
use rusttyc::TcKey;

use crate::{TyCheckState, TypeCheck, TypeCheckArg};

impl TypeCheck for TableExprNode {
    fn do_typecheck(&self, _: &[TypeCheckArg], ty_state: &mut TyCheckState) -> Option<TcKey> {
        if let Some(strct_name) = self.strct_name() {
            if let Some(strct) = ty_state
                .expect_strct_from_usage(strct_name.text(), strct_name.to_item())
                .cloned()
            {
                let col_tys: Vec<_> = strct
                    .field_keys
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(&a.field_num, &b.field_num))
                    .map(|field| field.ty.clone())
                    .collect();

                for row in self.rows() {
                    if row.values().count() < col_tys.len() {
                        ty_state.push_err(TyErr::TableRowToFewCol { row: row.to_item() }.into())
                    }

                    for (col_i, value) in row.values().enumerate() {
                        if let Some(val_key) = value.typecheck(ty_state) {
                            if let Some(col_ty) = col_tys.get(col_i) {
                                ty_state.equate_keys(val_key, col_ty.clone());
                            } else {
                                ty_state.push_err(
                                    TyErr::TableRowToManyCol { row: row.to_item() }.into(),
                                );
                            }
                        } else {
                            warn!("Not pushing error as prob error happend before");
                        }
                    }
                }

                let own_key = ty_state.new_term_key_concretiziesd(
                    self.to_item(),
                    ValueType::new_array(ValueType::Unspecified, self.to_item()),
                );
                ty_state.equate_keys(own_key, strct.self_key);
                return Some(own_key);
            }
        }

        // Okay incomplete input
        Some(ty_state.new_term_key_concretiziesd(
            self.to_item(),
            ValueType::new_array(ValueType::Unspecified, self.to_item()),
        ))
    }
}
