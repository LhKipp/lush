use std::sync::Arc;

use crate::{Command, ModPath, Scope, Value, ValueType, Variable};
use derive_builder::Builder;
use derive_new::new;
use lu_error::{LuResult, SourceCodeItem};
use lu_syntax::ast::{ArgSignatureNode, FlagSignatureNode, FnStmtNode, SignatureNode};
use lu_syntax::AstNode;
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

pub type ArgDecl = SourceCodeItem;

#[derive(Clone, Debug, Hash, new, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArgSignature {
    pub name: String,
    pub ty: ValueType,
    #[new(default)] // TODO this default should be false, making every flag necessary
    pub is_opt: bool, // TODO this is prob a bad idea???
    pub decl: ArgDecl,
}

impl ArgSignature {
    pub fn void(decl: ArgDecl) -> ArgSignature {
        ArgSignature::new("unused".into(), ValueType::Nil, decl)
    }

    /// ArgSignature with default in name
    pub fn in_(ty: ValueType, decl: ArgDecl) -> ArgSignature {
        ArgSignature::new(IN_ARG_NAME.into(), ty, decl)
    }

    /// ArgSignature with default ret name
    pub fn ret(ty: ValueType, decl: ArgDecl) -> ArgSignature {
        ArgSignature::new(RET_ARG_NAME.into(), ty, decl)
    }

    pub fn from_node(
        n: Option<ArgSignatureNode>,
        fallback_name: &str,
        fallback_decl: ArgDecl,
    ) -> Self {
        let name = n.as_ref().map(|n| n.name()).unwrap_or(fallback_name.into());
        let fallback_ty = ValueType::Unspecified;
        let decl: ArgDecl = n
            .as_ref()
            .map(|n| n.to_item())
            .unwrap_or_else(|| fallback_decl.into());
        let ty = n
            .as_ref()
            .map(|in_node| {
                in_node.type_().map(|ty| {
                    // Ty should always be some
                    ValueType::from_node(&ty.into_type())
                })
            })
            .flatten()
            .unwrap_or(fallback_ty); // or if in is not specified, use fallback

        ArgSignature::new(name, ty, decl)
    }

    pub fn to_var(&self) -> Variable {
        Variable::new(self.name.clone(), Value::Nil, self.decl.clone().into())
    }
}

#[derive(Clone, Debug, new, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlagSignature {
    pub long_name: Option<String>,
    pub short_name: Option<char>,
    pub ty: ValueType,
    #[new(default)] // TODO this default should be false, making every flag necessary
    pub is_opt: bool,
    #[serde(skip)]
    pub decl: Option<FlagSignatureNode>,
}

#[derive(Clone, Debug, new, Builder, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Signature {
    #[builder(default)]
    pub args: Vec<ArgSignature>,
    #[builder(setter(strip_option), default)]
    pub var_arg: Option<ArgSignature>,
    #[builder(default)]
    pub flags: Vec<FlagSignature>,
    pub in_arg: ArgSignature,
    pub ret_arg: ArgSignature,

    pub decl: SourceCodeItem,
}

impl Signature {
    pub fn default_signature(fn_sign_node_decl: SourceCodeItem) -> Signature {
        Signature::new(
            Vec::new(),
            Some(ArgSignature::new(
                VAR_ARGS_DEF_NAME.into(),
                ValueType::Any,
                fn_sign_node_decl.clone().into(),
            )),
            Vec::new(),
            ArgSignature::new(
                IN_ARG_NAME.into(),
                ValueType::Unspecified,
                fn_sign_node_decl.clone().into(),
            ),
            ArgSignature::new(
                RET_ARG_NAME.into(),
                ValueType::Unspecified,
                fn_sign_node_decl.clone().into(),
            ),
            fn_sign_node_decl,
        )
    }

    pub fn from_sign_and_stmt(
        sign_node: Option<SignatureNode>,
        fn_signature_decl: SourceCodeItem,
    ) -> Signature {
        if let Some(sign_node) = sign_node {
            Signature::source_signature(sign_node, fn_signature_decl)
        } else {
            Signature::default_signature(fn_signature_decl)
        }
    }

    pub fn source_signature(
        sign_node: SignatureNode,
        fallback_arg_decl: SourceCodeItem,
    ) -> Signature {
        let in_ty =
            ArgSignature::from_node(sign_node.in_arg(), IN_ARG_NAME, fallback_arg_decl.clone());
        let ret_ty = ArgSignature::from_node(sign_node.ret_arg(), RET_ARG_NAME, fallback_arg_decl);

        let args: Vec<ArgSignature> = sign_node
            .args()
            .iter()
            .map(|arg_node| -> ArgSignature {
                let arg_name = arg_node.name();
                let ty = arg_node
                    .type_()
                    .map(|ty_node| ValueType::from_node(&ty_node.into_type()))
                    .unwrap_or(ValueType::Unspecified);
                ArgSignature::new(arg_name, ty, arg_node.to_item())
            })
            .collect();
        let flags = sign_node
            .flags()
            .map(|flag_node| -> FlagSignature {
                let long_name = flag_node.long_name();
                let short_name = flag_node.short_name();
                let ty = flag_node
                    .type_()
                    .map(|ty_node| ValueType::from_node(&ty_node.into_type()))
                    .unwrap_or(ValueType::Unspecified);
                FlagSignature::new(long_name, short_name, ty, Some(flag_node))
            })
            .collect();
        let var_arg = sign_node.var_arg().map(|var_arg_node| {
            let name = var_arg_node.name();
            let ty = var_arg_node
                .type_()
                .map(|ty_node| ValueType::from_node(&ty_node.into_type()))
                .unwrap_or(ValueType::Any);
            ArgSignature::new(name, ty, var_arg_node.to_item())
        });
        Signature::new(args, var_arg, flags, in_ty, ret_ty, sign_node.to_item())
    }
}

/// Function is a struct containing all needed information for a function/closure
/// This should allow for less lookup in the ast later on
#[derive(Educe)]
#[educe(Debug)]
#[derive(Clone)]
pub struct Function {
    pub name: String,
    /// A signature is always present (if not user provided, defaulted.)
    pub signature: Signature,
    pub fn_node: FnStmtNode,
    // For closures only
    pub captured_vars: Vec<Variable>,

    /// Set when function is inserted into scope
    pub parent_module: ModPath,
}

impl Function {
    pub fn new(
        name: String,
        signature: Signature,
        fn_node: FnStmtNode,
        source_file_id: ModPath,
    ) -> Self {
        Self {
            name,
            signature,
            fn_node,
            captured_vars: Vec::new(),
            parent_module: source_file_id,
        }
    }
}

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run_cmd(&self, _: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<Value> {
        unreachable!(
            r#"
            Can't have evaluate/fn_stmt here, as that would require knowledge of eval here.
            This would lead to a circular dependency 
            (lu_interpreter_structs -> evaluate)
            (evaluate -> lu_interpreter_structs)
            Therefore we hack around the interface and provide the Command::as_function interface
            so that evaluate/cmd_stmt can react to this particular situation.
            This is isn't optimal, but the best solution
            "#
        );
    }

    fn as_function(&self) -> Option<&Function> {
        Some(self)
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn signature_item(&self) -> SourceCodeItem {
        self.fn_node.decl_item()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        Some(&self.parent_module)
    }
}
