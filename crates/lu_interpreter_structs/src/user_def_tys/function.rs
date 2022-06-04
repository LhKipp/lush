use crate::{
    CmdAttribute, CmdAttributeVariant, Command, FlagVariant, ModPath, SyScope, Value, ValueType,
    Variable,
};
use derive_builder::Builder;
use derive_more::From;
use derive_new::new;
use log::trace;
use lu_error::{lu_source_code_item, LuResult, SourceCodeItem};
use lu_syntax::ast::{
    ArgSignatureNode, ClosureExprNode, FnStmtNode, ImpureKeywordToken, MathExprNode, SignatureNode,
};
use lu_syntax::{AstNode, AstToken};
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, new, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArgSignature {
    pub name: String,
    pub ty: ValueType,
    pub is_opt: bool, // TODO this is prob a bad idea???
    pub decl: SourceCodeItem,
}

impl ArgSignature {
    pub fn req(name: String, ty: ValueType, decl: SourceCodeItem) -> Self {
        Self::new(name, ty, false, decl)
    }
    pub fn opt(name: String, ty: ValueType, decl: SourceCodeItem) -> Self {
        Self::new(name, ty, true, decl)
    }

    pub fn void(decl: SourceCodeItem) -> ArgSignature {
        ArgSignature::req("unused".into(), ValueType::Nil, decl)
    }

    /// ArgSignature with default in name
    pub fn in_(ty: ValueType, decl: SourceCodeItem) -> ArgSignature {
        ArgSignature::req(IN_ARG_NAME.into(), ty, decl)
    }

    /// ArgSignature with default ret name
    pub fn ret(ty: ValueType, decl: SourceCodeItem) -> ArgSignature {
        ArgSignature::req(RET_ARG_NAME.into(), ty, decl)
    }

    pub fn from_node(
        n: Option<ArgSignatureNode>,
        fallback_name: &str,
        fallback_decl: SourceCodeItem,
    ) -> Self {
        let name = n.as_ref().map(|n| n.name()).unwrap_or(fallback_name.into());
        let fallback_ty = ValueType::Unspecified;
        let decl: SourceCodeItem = n
            .as_ref()
            .map(|n| n.to_item())
            .unwrap_or_else(|| fallback_decl.into());
        let ty = n
            .as_ref()
            .map(|in_node| {
                in_node.type_().map(|ty| {
                    // Ty should always be some
                    ValueType::from_node(&ty)
                })
            })
            .flatten()
            .unwrap_or(fallback_ty); // or if in is not specified, use fallback

        ArgSignature::req(name, ty, decl)
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
    pub is_opt: bool,
    pub decl: SourceCodeItem,
}

impl FlagSignature {
    pub fn opt(
        long_name: Option<String>,
        short_name: Option<char>,
        ty: ValueType,
        decl: SourceCodeItem,
    ) -> Self {
        Self::new(long_name, short_name, ty, true, decl)
    }
    pub fn is_named_by(&self, name: &str) -> bool {
        let mut result = false;
        if let Some(long_name) = &self.long_name {
            result = result || name == long_name
        }
        if let Some(short_name) = &self.short_name {
            result = result || name.len() == 1 && &name.chars().next().unwrap() == short_name
        }
        result
    }

    pub fn best_name(&self) -> String {
        self.long_name
            .clone()
            .or(self.short_name.map(|c| c.to_string()))
            .unwrap()
    }
    pub fn is_required(&self) -> bool {
        !self.is_opt
    }
    pub fn to_var(&self) -> Variable {
        let name = self
            .long_name
            .clone()
            .or(self.short_name.map(|c| c.to_string()))
            .expect("Either long or shortname set");
        Variable::new(name, Value::Nil, self.decl.clone())
    }
}

#[derive(Clone, Debug, new, Builder, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Signature {
    #[builder(default)]
    pub args: Vec<ArgSignature>,
    #[builder(setter(strip_option), default)]
    pub var_arg: Option<ArgSignature>,
    #[builder(default)]
    pub flags: Vec<FlagSignature>,
    #[builder(default = "Signature::default_in_ret_void_arg()")]
    pub in_arg: ArgSignature,
    #[builder(default = "Signature::default_in_ret_void_arg()")]
    pub ret_arg: ArgSignature,

    pub decl: SourceCodeItem,
}

impl Signature {
    fn default_in_ret_void_arg() -> ArgSignature {
        ArgSignature::req("Unused".to_string(), ValueType::Nil, lu_source_code_item!())
    }
    pub fn req_flags(&self) -> Vec<FlagVariant> {
        self.flags
            .iter()
            .filter(|flag_sign| flag_sign.is_required())
            .map(|flag_sign| {
                if let Some(long_flag_name) = &flag_sign.long_name {
                    FlagVariant::LongFlag(long_flag_name.clone())
                } else if let Some(short_flag_name) = flag_sign.short_name {
                    FlagVariant::ShortFlag(short_flag_name)
                } else {
                    unreachable!()
                }
            })
            .collect()
    }

    pub fn default_signature(fn_sign_node_decl: SourceCodeItem) -> Signature {
        Signature::new(
            Vec::new(),
            Some(ArgSignature::req(
                VAR_ARGS_DEF_NAME.into(),
                ValueType::Any,
                fn_sign_node_decl.clone().into(),
            )),
            Vec::new(),
            ArgSignature::req(
                IN_ARG_NAME.into(),
                ValueType::Unspecified,
                fn_sign_node_decl.clone().into(),
            ),
            ArgSignature::req(
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
                let is_optional = arg_node.opt_modifier().is_some();
                let ty = arg_node
                    .type_()
                    .map(|ty_node| ValueType::from_node(&ty_node))
                    .unwrap_or(ValueType::Unspecified);
                ArgSignature::new(arg_name, ty, is_optional, arg_node.to_item())
            })
            .collect();
        let flags = sign_node
            .flags()
            .map(|flag_node| -> FlagSignature {
                let long_name = flag_node.long_name();
                let short_name = flag_node.short_name();
                let ty = flag_node
                    .type_()
                    .map(|ty_node| ValueType::from_node(&ty_node))
                    .unwrap_or(ValueType::Bool); // Flags have a default ty of bool.
                let optional = !flag_node.is_required();
                FlagSignature::new(long_name, short_name, ty, optional, flag_node.to_item())
            })
            .collect();
        let var_arg = sign_node.var_arg().map(|var_arg_node| {
            let name = var_arg_node.name();
            let ty = var_arg_node
                .type_()
                .map(|ty_node| ValueType::from_node(&ty_node))
                .unwrap_or(ValueType::Any);
            ArgSignature::req(name, ty, var_arg_node.to_item())
        });
        let sign = Signature::new(args, var_arg, flags, in_ty, ret_ty, sign_node.to_item());
        trace!("Generated Signature: {:#?}", sign);
        sign
    }
}

/// A node in the ast which is evaluable as a cmd
#[derive(From, Debug, Clone)]
pub enum CmdEvaluableNode {
    FnStmt(FnStmtNode),
    ClsExpr(ClosureExprNode),
    MathExpr(MathExprNode),
}
impl CmdEvaluableNode {
    pub fn to_item(&self) -> SourceCodeItem {
        match self {
            CmdEvaluableNode::FnStmt(n) => n.decl_item(),
            CmdEvaluableNode::ClsExpr(n) => n.decl_item(),
            CmdEvaluableNode::MathExpr(n) => n.to_item(),
        }
    }
    pub fn name(&self) -> Option<String> {
        match self {
            CmdEvaluableNode::FnStmt(n) => n.name(),
            CmdEvaluableNode::ClsExpr(_) => None,
            CmdEvaluableNode::MathExpr(_) => None,
        }
    }
}
/// Function is a struct containing all needed information for a function/closure
/// This should allow for less lookup in the ast later on (and easier handling of funcs)
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    /// A signature is always present (if not user provided, defaulted.)
    pub signature: Signature,
    pub fn_node: CmdEvaluableNode,
    // For closures only
    pub captured_vars: Vec<Variable>,

    pub attributes: Vec<CmdAttribute>,

    /// Set when function is inserted into scope
    /// Some for regular Functions, none for closures
    pub parent_module: Option<ModPath>,
}

impl Function {
    pub fn func_from_node(fn_stmt: FnStmtNode, source_file_id: ModPath) -> Function {
        let name = fn_stmt.name().expect("Functions have a name");
        Function::from_node(fn_stmt.into(), name, Some(source_file_id))
    }

    pub fn closure_from_node(fn_stmt: CmdEvaluableNode) -> Function {
        assert!(fn_stmt.name().is_none(), "Closures dont have a name");
        let name = Function::closure_name_from_node(&fn_stmt);
        Function::from_node(
            fn_stmt, // Provide internal name for closures
            name, None,
        )
    }

    pub fn closure_name_from_node(cls_stmt: &CmdEvaluableNode) -> String {
        format!("closure_at_{:?}", cls_stmt.to_item().range)
    }

    fn from_node(
        fn_stmt: CmdEvaluableNode,
        name: String,
        source_file_id: Option<ModPath>,
    ) -> Function {
        // Source the signature (either user provided or default)
        let sign = match &fn_stmt {
            CmdEvaluableNode::FnStmt(fn_stmt) => {
                Signature::from_sign_and_stmt(fn_stmt.signature(), fn_stmt.decl_item())
            }
            CmdEvaluableNode::ClsExpr(cls_expr) => {
                Signature::from_sign_and_stmt(cls_expr.signature(), cls_expr.decl_item())
            }
            CmdEvaluableNode::MathExpr(math_expr) => {
                Signature::default_signature(math_expr.to_item())
            }
        };

        let attrs = Self::attrs_from_node(&fn_stmt);

        Self {
            name,
            signature: sign,
            fn_node: fn_stmt,
            captured_vars: Vec::new(),
            parent_module: source_file_id,
            attributes: attrs,
        }
    }

    fn attrs_from_node(cmd_node: &CmdEvaluableNode) -> Vec<CmdAttribute> {
        let mut attrs = vec![];
        let mut cls_fn_handler = |impure_attr: &Option<ImpureKeywordToken>| {
            if let Some(impure_token) = impure_attr {
                attrs.push(CmdAttribute::new(
                    CmdAttributeVariant::Impure,
                    impure_token.to_item(),
                ));
            } else {
                // By default all lu-functions are pure :)
                // This is okay, as there will be a warning for all impure function calls
                attrs.push(CmdAttribute::new(
                    CmdAttributeVariant::Pure,
                    lu_source_code_item!(),
                ));
            }
        };
        match cmd_node {
            CmdEvaluableNode::FnStmt(fn_stmt) => cls_fn_handler(&fn_stmt.impure_attr()),
            CmdEvaluableNode::ClsExpr(cls_expr) => cls_fn_handler(&cls_expr.impure_attr()),
            CmdEvaluableNode::MathExpr(_) => attrs.push(CmdAttribute::new(
                CmdAttributeVariant::Pure,
                lu_source_code_item!(),
            )),
        }
        attrs
    }
}

impl Command for Function {
    fn name(&self) -> &str {
        &self.name
    }

    fn do_run_cmd(&self, _: &mut SyScope) -> LuResult<Value> {
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

    fn as_function_mut(&mut self) -> Option<&mut Function> {
        Some(self)
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn signature_item(&self) -> SourceCodeItem {
        self.fn_node.to_item()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        self.parent_module.as_ref()
    }

    fn attributes(&self) -> &[crate::CmdAttribute] {
        &self.attributes
    }
}
