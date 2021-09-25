use std::fmt::Display;

use enum_as_inner::EnumAsInner;
use log::warn;
use lu_error::LuErr;
use lu_syntax::{ast::LuTypeSpecifierElement, AstNode};
use rusttyc::{types::Arity, Constructable, Partial, Variant as TcVariant};
use serde::{Deserialize, Serialize};

use crate::Signature;

fn cmp_sign_types(_: &Signature, _: &Signature) -> bool {
    todo!()
    // a.in_type == b.in_type && a.ret_type == b.ret_type &&
    //     a.iter
}
// #[derive(Educe)]
// #[educe(Hash)]
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// // We need something being able to Serialize, Deserialize. Therefore we don't reuse Signature here
// pub struct FuncType {
//     in_ty: ValueType,
//     ret_ty: ValueType,
//     args_ty: Vec<ValueType>,
//     var_arg_ty: Option<ValueType>,
//     #[educe(Hash(ignore))]
//     flags_ty: Vec<FlagSignature>,
// }
// #[derive(Educe)] // TODO educe partial eq not working
// #[educe(PartialEq)]
#[derive(Clone, Debug, Serialize, Deserialize, Hash, EnumAsInner, Eq)]
pub enum ValueType {
    /// Variant to indicate an already occured error. Error acts like any does, but
    /// further ty_checking does not generate errors based on this ValueType::Error
    Error,
    /// Unspecified, but can be refined by any other type during type_checking
    Unspecified,
    /// Type that can be of any type and will not be restricted
    Any,
    /// Type to indicate the emptiness // TODO is this the same as Nil?
    Void,
    /// The empty void type
    Nil,
    Bool,
    Number,
    String,
    BareWord,
    Array(Box<ValueType>),
    // #[educe(PartialEq(method = "cmp_sign_types"))]
    Func(Box<Signature>),
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::Error, ValueType::Error) => true,
            (ValueType::Unspecified, ValueType::Unspecified) => true,
            (ValueType::Any, ValueType::Any) => true,
            (ValueType::Void, ValueType::Void) => true,
            (ValueType::Nil, ValueType::Nil) => true,
            (ValueType::Bool, ValueType::Bool) => true,
            (ValueType::Number, ValueType::Number) => true,
            (ValueType::String, ValueType::String) => true,
            (ValueType::BareWord, ValueType::BareWord) => true,
            (ValueType::Array(a_inner), ValueType::Array(b_inner)) => a_inner == b_inner,
            (ValueType::Func(a_sign), ValueType::Func(b_sign)) => cmp_sign_types(a_sign, b_sign),
            _ => false,
        }
    }
}

impl ValueType {
    pub fn new_array(inner_ty: ValueType) -> Self {
        ValueType::Array(Box::new(inner_ty))
    }

    pub fn new_func(sign: Signature) -> Self {
        ValueType::Func(Box::new(sign))
    }

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
            LuTypeSpecifierElement::FnType(fn_ty) => {
                let (sign, errs) =
                    Signature::from_sign_and_stmt(fn_ty.signature(), fn_ty.to_item());
                if !errs.is_empty() {
                    todo!("Return (valuety, err)");
                }
                ValueType::new_func(sign)
            }
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
            | ValueType::Void
            | ValueType::Any
            | ValueType::Nil
            | ValueType::Bool
            | ValueType::Number
            | ValueType::String
            | ValueType::Func(_)
            | ValueType::BareWord => Arity::Fixed(0),
            ValueType::Array(_) => Arity::Fixed(1),
            ValueType::Error => Self::arity(&ValueType::Any),
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
            ValueType::Void => todo!(),
        }
    }
}
