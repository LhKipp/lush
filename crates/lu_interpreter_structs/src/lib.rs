#[macro_use]
extern crate derive_is_enum_variant;
extern crate derive_more;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate educe;

mod command;
mod command_collection;
pub mod dbg_state;
mod evaluate;
mod flag;
mod module;
pub mod prelude;
mod scope;
pub mod special_scope_vars;
mod use_path;
mod user_def_tys;
mod value;
mod value_type;
mod variable;

pub use command::{CmdAttribute, CmdAttributeVariant, Command};
pub use command_collection::CommandCollection;
pub use evaluate::*;
pub use flag::*;
pub use module::ModInfo;
pub use scope::{Scope, ScopeFrame, ScopeFrameId, ScopeFrameState, ScopeFrameTag, SyScope};
pub use special_scope_vars::*;
pub use use_path::{ModPath, ModPathVariant, UsePath};
pub use user_def_tys::*;
pub use value::Value;
pub use value_type::{ValueType, ValueTypeErr};
pub use variable::{VarDeclNode, Variable};
