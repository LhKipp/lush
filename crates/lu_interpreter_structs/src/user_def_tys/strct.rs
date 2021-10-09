use derive_new::new;
use lu_error::SourceCodeItem;
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
    pub fn from_node(field_node: &StrctFieldNode) -> StrctField {
        let name = field_node.name();
        let decl = field_node.to_item();
        let ty = field_node
            .ty()
            .map(|ty_node| ty_node.into_type())
            .map(|ty_spec| ValueType::from_node(&ty_spec))
            .unwrap_or(ValueType::Unspecified);

        StrctField::new(name, ty, decl)
    }
}

#[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Strct {
    pub name: String,
    pub fields: Vec<StrctField>,
    pub decl: SourceCodeItem,
}
