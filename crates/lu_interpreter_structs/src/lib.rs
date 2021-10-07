#[macro_use]
extern crate derive_is_enum_variant;
extern crate derive_more;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate educe;

mod command;
pub mod prelude;
mod scope;
mod use_path;
mod user_def_tys;
mod value;
mod value_type;
mod variable;

pub use command::Command;
pub use scope::{Scope, ScopeFrame, ScopeFrameId, ScopeFrameState, ScopeFrameTag};
pub use use_path::{UsePath, UsePathVariant};
pub use user_def_tys::*;
pub use value::Value;
pub use value_type::{ValueType, ValueTypeErr};
pub use variable::{VarDeclNode, Variable};
