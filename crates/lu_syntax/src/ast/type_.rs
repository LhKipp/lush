use lu_value::ValueType;

use crate::{AstNode, AstToken};

use super::{support, ArrayTypeNode, LuTypeNode, LuTypeSpecifierElement};

impl LuTypeNode {
    pub fn into_type(&self) -> ValueType {
        match support::element_child::<LuTypeSpecifierElement>(self.syntax()).unwrap() {
            LuTypeSpecifierElement::NumberKeyword(_) => ValueType::Number,
            LuTypeSpecifierElement::AnyKeyword(_) => ValueType::Any,
            LuTypeSpecifierElement::NilKeyword(_) => ValueType::Nil,
            LuTypeSpecifierElement::BoolKeyword(_) => ValueType::Bool,
            LuTypeSpecifierElement::StringKeyword(_) => ValueType::String,
            LuTypeSpecifierElement::FnKeyword(_) => ValueType::Function,
            LuTypeSpecifierElement::ArrayType(n) => n.into_type(),
            LuTypeSpecifierElement::BareWord(n) => ValueType::Unresolved(n.text().to_string()),
        }
    }
}

impl ArrayTypeNode {
    pub fn into_type(&self) -> ValueType {
        let inner_t = support::node_child::<LuTypeNode>(self.syntax())
            .map_or(ValueType::Unspecified, |n| n.into_type());
        ValueType::Array(Box::new(inner_t))
    }
}
