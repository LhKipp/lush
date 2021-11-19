use derive_new::new;
use itertools::Itertools;
use lu_error::SourceCodeItem;
use lu_syntax::ast::StrctFieldNode;
use lu_syntax::AstNode;

use crate::ValueType;
use serde::{Deserialize, Serialize};

#[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct StrctField {
    pub name: String,
    pub ty: ValueType,
    pub field_num: u32,
    pub decl: SourceCodeItem,
}

impl StrctField {
    pub fn from_node(field_node: &StrctFieldNode, field_num: u32) -> StrctField {
        let name = field_node.name();
        let decl = field_node.to_item();
        let ty = field_node
            .ty()
            .map(|ty_node| ValueType::from_node(&ty_node))
            .unwrap_or(ValueType::Unspecified);

        StrctField::new(name, ty, field_num, decl)
    }
}

#[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Strct {
    pub name: String,
    pub fields: Vec<StrctField>,
    pub decl: SourceCodeItem,
}

impl Strct {
    pub fn fields_sorted_by_order(&self) -> Vec<&StrctField> {
        self.fields
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.field_num, &b.field_num))
            .collect()
    }
}
