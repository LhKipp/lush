use lu_error::LuErr;
use lu_syntax::ast::LuTypeSpecifierElement;
use rusttyc::{types::Arity, Partial, Variant as TcVariant};
use serde::{Deserialize, Serialize};

use super::Resolver;

// enum ParamType{
//     GenericT(i32),
//     Concrete(ValueType),
// }

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// struct FuncType{
//     name: String,
//     req_flags: Vec<String>,

//     args: Vec<ParamType>,
//     ret_t: ParamType
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ValueType {
    /// Variant to indicate an already occured error. Error acts like any does, but further
    /// resolution should not generate further errors
    Error,
    Unspecified,
    Any,
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array(Box<ValueType>),
    // Function(FuncType),
}

impl ValueType {
    pub fn from_node(
        node: &LuTypeSpecifierElement,
        resolver: &Resolver,
    ) -> Result<ValueType, LuErr> {
        let ty = match node {
            LuTypeSpecifierElement::AnyKeyword(_) => ValueType::Any,
            LuTypeSpecifierElement::NumberKeyword(_) => ValueType::Number,
            LuTypeSpecifierElement::NilKeyword(_) => ValueType::Nil,
            LuTypeSpecifierElement::BoolKeyword(_) => ValueType::Bool,
            LuTypeSpecifierElement::StringKeyword(_) => ValueType::String,
            LuTypeSpecifierElement::BareWord(_) => ValueType::BareWord,
            LuTypeSpecifierElement::ArrayType(arr) => {
                if let Some(inner) = arr.inner_type() {
                    ValueType::from_node(&inner.into_type(), resolver)?
                } else {
                    ValueType::Unspecified
                }
            }
            _ => todo!(),
            // LuTypeSpecifierElement::FnKeyword(_) => ValueType::F,;
        };
        Ok(ty)
    }
}

impl TcVariant for ValueType {
    type Err = String;

    fn top() -> Self {
        ValueType::Any
    }

    fn meet(lhs: Partial<Self>, rhs: Partial<Self>) -> Result<Partial<Self>, Self::Err> {
        let ty = if lhs.variant == rhs.variant {
            lhs.variant
        } else {
            // Not equal check for special coercion rules
            let coercable_ty = match (&lhs.variant, &rhs.variant) {
                (ValueType::Any, other) | (other, ValueType::Any) => Some(other.clone()),
                (ValueType::String, ValueType::BareWord) => Some(ValueType::String),
                (ValueType::BareWord, ValueType::String) => Some(ValueType::String),
                (ValueType::Array(lhs_inner), ValueType::Array(rhs_inner)) => {
                    let (lhs_arity, rhs_arity) = match (lhs_inner.arity(), rhs_inner.arity()) {
                        (Arity::Fixed(l), Arity::Fixed(r)) => (l, r),
                        _ => unreachable!("All types have fixed arity"),
                    };
                    let inner = ValueType::meet(
                        Partial {
                            variant: *lhs_inner.clone(),
                            least_arity: lhs_arity,
                        },
                        Partial {
                            variant: *rhs_inner.clone(),
                            least_arity: rhs_arity,
                        },
                    )?;
                    Some(ValueType::Array(Box::new(inner.variant)))
                }
                _ => None,
            };
            coercable_ty.ok_or_else(|| {
                format!(
                    "{:?} can not be combined with {:?}",
                    lhs.variant, rhs.variant
                )
                .to_string()
            })?
        };

        let arity = match ty.arity() {
            Arity::Variable => unreachable!("All types have fixed arity"),
            Arity::Fixed(arity) => arity,
        };

        Ok(Partial {
            variant: ty,
            least_arity: arity,
        })
    }

    fn arity(&self) -> Arity {
        match self {
            ValueType::Unspecified
            | ValueType::Any
            | ValueType::Nil
            | ValueType::Bool
            | ValueType::Number
            | ValueType::String
            | ValueType::BareWord => Arity::Fixed(0),
            ValueType::Array(_) => Arity::Fixed(1),

            ValueType::Error => Self::arity(&ValueType::Any),
        }
    }
}
