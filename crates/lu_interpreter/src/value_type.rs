use std::{collections::HashMap, fmt::Display};

use log::warn;
use lu_error::LuErr;
use lu_syntax::ast::LuTypeSpecifierElement;
use rusttyc::{types::Arity, Constructable, Partial, Variant as TcVariant};
use serde::{Deserialize, Serialize};

use crate::FlagSignature;

#[derive(Educe)]
#[educe(Hash)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FuncType {
    in_ty: Option<ValueType>,
    ret_ty: Option<ValueType>,
    args_ty: Vec<ValueType>,
    var_arg_ty: Option<ValueType>,
    #[educe(Hash(ignore))]
    flags_ty: HashMap<FlagSignature, ValueType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ValueType {
    /// Variant to indicate an already occured error. Error acts like any does, but
    /// further ty_checking does not generate errors based on this ValueType::Error
    Error,
    Unspecified,
    /// Any that can be only of one type
    Any,
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array(Box<ValueType>),
    Func(Box<FuncType>),
}

impl ValueType {
    pub fn from_node(node: &LuTypeSpecifierElement) -> Result<ValueType, LuErr> {
        let ty = match node {
            LuTypeSpecifierElement::AnyKeyword(_) => {
                warn!("RETURNING WRONG VALUE_TYPE: Any INSTEAD OF AnyOf");
                ValueType::Any // TODO this must be AnyOf!!!
            }
            LuTypeSpecifierElement::NumberKeyword(_) => ValueType::Number,
            LuTypeSpecifierElement::NilKeyword(_) => ValueType::Nil,
            LuTypeSpecifierElement::BoolKeyword(_) => ValueType::Bool,
            LuTypeSpecifierElement::StringKeyword(_) => ValueType::String,
            LuTypeSpecifierElement::BareWord(_) => ValueType::BareWord,
            LuTypeSpecifierElement::ArrayType(arr) => {
                if let Some(inner) = arr.inner_type() {
                    ValueType::from_node(&inner.into_type())?
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

#[derive(Clone, Debug)]
pub enum ValueTypeErr {
    Dummy,
    /// Lhs not meetable with rhs
    NotMeetAble {
        lhs_ty: ValueType,
        rhs_ty: ValueType,
    },
}

impl TcVariant for ValueType {
    type Err = ValueTypeErr;

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
            coercable_ty.ok_or_else(|| ValueTypeErr::NotMeetAble {
                lhs_ty: lhs.variant,
                rhs_ty: rhs.variant,
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
            ValueType::Func(_) => todo!(),
        }
    }
}

impl Constructable for ValueType {
    type Type = ValueType;

    fn construct(&self, _: &[Self::Type]) -> Result<Self::Type, <Self as TcVariant>::Err> {
        Ok(self.clone())
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Error => write!(f, "ERROR"),
            ValueType::Unspecified => write!(f, "UNSPECIFIED"),
            ValueType::Any => write!(f, "any"),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Bool => write!(f, "bool"),
            ValueType::Number => write!(f, "num"),
            ValueType::String => write!(f, "str"),
            ValueType::BareWord => write!(f, "bare_word"),
            ValueType::Array(t) => write!(f, "[{}]", t),
            ValueType::Func(_) => todo!(),
        }
    }
}
