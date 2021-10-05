use derive_new::new;
use lu_error::{LuErr, SourceCodeItem};
use lu_syntax::ast::StrctFieldNode;
use lu_syntax::AstNode;

use crate::ValueType;
use serde::{Deserialize, Serialize};

#[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct StrctField {
    pub name: String,
    pub ty: ValueType,
    pub decl: SourceCodeItem,
}

impl StrctField {
    pub fn from_node(field_node: &StrctFieldNode) -> (StrctField, Vec<LuErr>) {
        let name = field_node.name();
        let decl = field_node.to_item();
        let fallback_ty = (ValueType::Unspecified, vec![]);
        let ty = field_node
            .ty()
            .map(|ty_node| ty_node.into_type())
            .map(|ty_spec| {
                // Ty should always be some
                ValueType::from_node(&ty_spec)
                    .map_or_else(|err| (ValueType::Error, vec![err]), |ty| (ty, vec![]))
            })
            .unwrap_or(fallback_ty); // or if in is not specified, use fallback

        (StrctField::new(name, ty.0, decl), ty.1)
    }
}

#[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Strct {
    pub name: String,
    pub fields: Vec<StrctField>,
    pub decl: SourceCodeItem,
}
