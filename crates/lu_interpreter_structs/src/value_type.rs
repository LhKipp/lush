use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    sync::{Arc, Weak},
};

use enum_as_inner::EnumAsInner;
use log::{debug, warn};
use lu_error::{util::Outcome, LuErr, SourceCodeItem};
use lu_syntax::{ast::LuTypeSpecifierElement, AstElement, AstNode, AstToken};
use parking_lot::RwLock;
use rusttyc::{types::Arity, Constructable, Partial, Variant as TcVariant};
use serde::{Deserialize, Serialize, Serializer};

use crate::{Scope, Signature, Strct, Variable};

fn cmp_sign_types(_: &Box<Signature>, _: &Box<Signature>) -> bool {
    todo!()
    // a.in_type == b.in_type && a.ret_type == b.ret_type &&
    //     a.iter
}
fn cmp_strcts(a: &Weak<RwLock<Strct>>, b: &Weak<RwLock<Strct>>) -> bool {
    // TODO also compare that they are declared in same module !!!
    let (a, b) = (Weak::upgrade(a).unwrap(), Weak::upgrade(b).unwrap()); // TODO no unwrap
    let (l_a, l_b) = (a.read(), b.read());
    l_a.name == l_b.name
}
fn cmp_inner_tys(a: &Box<ValueType>, b: &Box<ValueType>) -> bool {
    *a == *b
}

fn hash_as_ptr<H: Hasher>(strct: &Weak<RwLock<Strct>>, state: &mut H) {
    Hash::hash(&strct.as_ptr(), state)
}

fn serialize_name_only<S>(_: &Weak<RwLock<Strct>>, _: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    //serialze_newtype_variant
    todo!();
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
#[derive(Educe, Clone, Debug, Serialize, Deserialize, EnumAsInner)]
#[educe(Hash, PartialEq)]
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
    /// e.G. T1, T2... Specially handled by engine
    Generic(String),
    Bool,
    Number,
    String,
    BareWord,
    /// Struct with name (Final Strct type when)
    #[serde(skip_deserializing)]
    #[serde(serialize_with = "serialize_name_only")]
    // #[serde(default = "deser_not_possible")]
    Strct(
        #[educe(Hash(method = "hash_as_ptr"))]
        #[educe(PartialEq(method = "cmp_strcts"))]
        Weak<RwLock<Strct>>,
    ),
    /// Before resolving all UsePaths, the correct strct behind a StrctName can not be determined.
    /// However we need to create a ValueType when sourcing functions etc. Therefore we introduce
    /// this temporary type
    StrctName(String),
    /// Box with inner ty and inner_ty_decl
    Array {
        #[educe(PartialEq(method = "cmp_inner_tys"))]
        inner_ty: Box<ValueType>,
        #[educe(PartialEq(ignore))]
        inner_ty_decl: SourceCodeItem,
    },
    Func(#[educe(PartialEq(method = "cmp_sign_types"))] Box<Signature>),
}

impl Eq for ValueType {}
// TODO remove this method and derive PartialEq
// impl PartialEq for ValueType {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (ValueType::Error, ValueType::Error) => true,
//             (ValueType::Unspecified, ValueType::Unspecified) => true,
//             (ValueType::Any, ValueType::Any) => true,
//             (ValueType::Void, ValueType::Void) => true,
//             (ValueType::Nil, ValueType::Nil) => true,
//             (ValueType::Bool, ValueType::Bool) => true,
//             (ValueType::Number, ValueType::Number) => true,
//             (ValueType::String, ValueType::String) => true,
//             (ValueType::BareWord, ValueType::BareWord) => true,
//             (ValueType::Array { inner_ty: a_ty, .. }, ValueType::Array { inner_ty: b_ty, .. }) => {
//                 a_ty == b_ty
//             }
//             (ValueType::Func(a_sign), ValueType::Func(b_sign)) => cmp_sign_types(a_sign, b_sign),
//             (ValueType::StrctName(a), ValueType::StrctName(b)) => a == b,
//             (ValueType::Generic(a), ValueType::Generic(b)) => a == b,
//             (a, b) => {
//                 warn!("Compared two value_types which are distinct: {} {}?", a, b);
//                 warn!("If these 2 types are the same, add a match arm here");
//                 false
//             }
//         }
//     }
// }

impl ValueType {
    pub fn new_array(inner_ty: ValueType, inner_ty_decl: SourceCodeItem) -> Self {
        ValueType::Array {
            inner_ty: Box::new(inner_ty),
            inner_ty_decl,
        }
    }

    pub fn new_func(sign: Signature) -> Self {
        ValueType::Func(Box::new(sign))
    }

    pub fn new_strct(strct: Weak<RwLock<Strct>>) -> Self {
        ValueType::Strct(strct)
    }

    pub fn from_node_or_err_ty(node: &LuTypeSpecifierElement) -> (ValueType, Option<LuErr>) {
        ValueType::from_node(node).map_or_else(|err| (ValueType::Error, Some(err)), |ty| (ty, None))
    }

    /// TODO this func feels like a halfway solution
    /// Function and Strct from_node can't use it (as it happens in the resolve step)
    /// Therefore these would neet a resolve_strct_names step after creation, called in typecheck
    ///
    /// A possible way around this awkward structure would be either:
    /// Option1: Create a mapping in the resolve step: LuTypeSpecifierElement => ???
    ///             which somehow magically gives the value_type
    /// Option2: do the resolve_strct_names step for typecheck and keep using this func (preferred
    /// now)
    pub fn from_node_or_err_resolve_strct_name(
        node: &LuTypeSpecifierElement,
        scope: &Scope<Variable>,
    ) -> Outcome<ValueType> {
        let (ty, err) = Self::from_node_or_err_ty(node);
        assert!(err.is_none()); // TODO use outcome interface in above funcs
        if let Self::StrctName(strct_name) = ty {
            let strct = scope
                .expect_strct(&strct_name, node.to_item())
                .map(|strct| Arc::downgrade(strct))
                .map(|strct| ValueType::Strct(strct));

            Outcome::from_result(strct, ValueType::Error)
        } else {
            Outcome::ok(ty)
        }
    }

    pub fn from_node(node: &LuTypeSpecifierElement) -> Result<ValueType, LuErr> {
        // TODO make return type (ValueType, Option<LuErr>)
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
            LuTypeSpecifierElement::GenericType(n) => ValueType::Generic(n.text().to_string()),
            LuTypeSpecifierElement::StrctName(n) => ValueType::StrctName(n.text().to_string()),
            LuTypeSpecifierElement::ArrayType(arr) => {
                let (inner_ty, inner_ty_decl) = if let Some(inner) = arr.inner_type() {
                    (ValueType::from_node(&inner.into_type())?, inner.to_item())
                } else {
                    (ValueType::Unspecified, arr.to_item())
                };
                ValueType::new_array(inner_ty, inner_ty_decl)
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
        ValueType::Unspecified
    }

    fn meet(lhs: Partial<Self>, rhs: Partial<Self>) -> Result<Partial<Self>, Self::Err> {
        assert!(
            lhs.variant.as_generic().is_none() && rhs.variant.as_generic().is_none(),
            "Generics have to be substituted before meet"
        );
        debug!("Meeting: {} {}", lhs.variant, rhs.variant);
        let ty = if lhs.variant == rhs.variant {
            lhs.variant
        } else {
            // Not equal check for special coercion rules
            let coercable_ty = match (&lhs.variant, &rhs.variant) {
                (ValueType::Unspecified, other) | (other, ValueType::Unspecified) => {
                    Some(other.clone())
                }
                (ValueType::Any, other) | (other, ValueType::Any) => Some(other.clone()),
                (ValueType::String, ValueType::BareWord) => Some(ValueType::String),
                (ValueType::BareWord, ValueType::String) => Some(ValueType::String),
                (
                    ValueType::Array {
                        inner_ty: lhs_inner,
                        inner_ty_decl: lhs_decl,
                    },
                    ValueType::Array {
                        inner_ty: rhs_inner,
                        ..
                    },
                ) => {
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
                    Some(ValueType::new_array(inner.variant, lhs_decl.clone())) // TODO the decl may be wrong for some meets
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

        debug!("Result of meet: {}", ty);
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
            | ValueType::Generic(_)
            | ValueType::Bool
            | ValueType::Number
            | ValueType::String
            | ValueType::Func(_)
            | ValueType::Strct(_)
            | ValueType::BareWord => Arity::Fixed(0),
            ValueType::Array { .. } => Arity::Fixed(1),
            ValueType::Error => Self::arity(&ValueType::Any),
            ValueType::StrctName(_) => unreachable!("Tmp type"),
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
            ValueType::Array { inner_ty, .. } => write!(f, "[{}]", *inner_ty),
            ValueType::Strct(strct) => {
                let strct = Weak::upgrade(strct).unwrap();
                let l_strct = strct.read();
                write!(f, "{}{{ ", l_strct.name)?;
                for field in &l_strct.fields {
                    write!(f, "{}: {}", field.name, field.ty)?;
                }
                write!(f, "}}")
            }
            ValueType::Func(_) => todo!(),
            ValueType::Void => write!(f, "void"),
            ValueType::Generic(name) => write!(f, "{}", name),
            ValueType::StrctName(name) => write!(f, "{}", name),
        }
    }
}
