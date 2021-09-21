#![allow(dead_code)]
use crate::scope::ScopeFrameId;
use crate::{Command, Evaluable, Variable};
use crate::{Evaluator, ValueType};
use derive_more::From;
use lu_error::{LuErr, SourceCodeItem};
use lu_syntax::ast::{ArgSignatureNode, FlagSignatureNode, FnStmtNode};
use lu_syntax::AstNode;
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};
use lu_value::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, From)]
pub enum Decl {
    Arg(ArgSignatureNode),
    FnNodeFallback(FnStmtNode),
}

impl Decl {
    pub fn into_item(&self) -> SourceCodeItem {
        match self {
            Decl::FnNodeFallback(fn_node) => fn_node.fallback_in_ret_item(),
            Decl::Arg(arg) => arg.into_item(),
        }
    }
}

#[derive(Clone, Debug, new)]
pub struct ArgSignature {
    pub name: String,
    pub type_: ValueType,
    #[new(default)] // TODO this default should be false, making every flag necessary
    pub is_opt: bool, // TODO this is prob a bad idea???
    pub decl: Decl,
}

impl ArgSignature {
    pub fn from_node(
        n: Option<ArgSignatureNode>,
        fallback_name: &str,
        fn_node: &FnStmtNode,
    ) -> (Self, Option<LuErr>) {
        let name = n.as_ref().map(|n| n.name()).unwrap_or(fallback_name.into());
        let fallback_ty = (ValueType::Unspecified, None);
        let decl: Decl = n
            .as_ref()
            .map(|n| n.clone().into())
            .unwrap_or_else(|| fn_node.clone().into());
        let ty = n
            .as_ref()
            .map(|in_node| {
                in_node
                    .type_()
                    .map(|ty| {
                        // Ty should always be some
                        ValueType::from_node(&ty.into_type())
                            .map_or_else(|err| (ValueType::Error, Some(err)), |ty| (ty, None))
                    })
                    .unwrap_or(fallback_ty.clone()) // But for incomplete input we fallback
            })
            .unwrap_or(fallback_ty); // or if in is not specified, use fallback

        (ArgSignature::new(name, ty.0, decl), ty.1)
    }
}

#[derive(Clone, Debug, new, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlagSignature {
    pub long_name: Option<String>,
    pub short_name: Option<char>,
    pub type_: ValueType,
    #[new(default)] // TODO this default should be false, making every flag necessary
    pub is_opt: bool,
    #[serde(skip)]
    pub decl: Option<FlagSignatureNode>,
}

#[derive(Clone, Debug, new)]
pub struct Signature {
    pub args: Vec<ArgSignature>,
    pub var_arg: Option<ArgSignature>,
    pub flags: Vec<FlagSignature>,
    pub in_type: ArgSignature,
    pub ret_type: ArgSignature,
}

impl Signature {
    pub fn default_signature(fn_node: &FnStmtNode) -> Signature {
        Signature::new(
            Vec::new(),
            Some(ArgSignature::new(
                VAR_ARGS_DEF_NAME.into(),
                ValueType::Any,
                fn_node.clone().into(),
            )),
            Vec::new(),
            ArgSignature::new(
                IN_ARG_NAME.into(),
                ValueType::Unspecified,
                fn_node.clone().into(),
            ),
            ArgSignature::new(
                RET_ARG_NAME.into(),
                ValueType::Unspecified,
                fn_node.clone().into(),
            ),
        )
    }
}

/// Function is a struct containing all needed information for a function/closure
/// This should allow for less lookup in the ast later on
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    /// A signature is always present (if not user provided, defaulted.)
    pub signature: Signature,
    pub fn_node: FnStmtNode,
    pub parent_frame_id: ScopeFrameId,
    // For closures only
    pub captured_vars: Vec<Variable>,
}

impl Function {
    pub fn new(
        name: String,
        signature: Signature,
        fn_node: FnStmtNode,
        parent_frame_id: ScopeFrameId,
    ) -> Self {
        Self {
            name,
            signature,
            parent_frame_id,
            fn_node,
            captured_vars: Vec::new(),
        }
    }
}

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run(
        &self,
        _: &[crate::EvalArg],
        state: &mut Evaluator,
    ) -> lu_error::LuResult<lu_value::Value> {
        // TODO typecheck and put vars into scope
        if let Some(block) = self.fn_node.block_stmt() {
            block.evaluate(state)
        } else {
            Ok(Value::Nil)
        }
    }
}
