use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct TyOfBuiltin {
    sign: Signature,
}

const TO_GET_TY_OF_ARG: &str = "value";
static TY_OF_BUILTIN_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl TyOfBuiltin {
    pub fn new() -> Self {
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(lu_source_code_item!())
            .args(vec![ArgSignature::new(
                TO_GET_TY_OF_ARG.to_string(),
                ValueType::Any,
                lu_source_code_item!(-3).into(),
            )])
            .ret_arg(ArgSignature::new(
                "value_type".into(),
                ValueType::String,
                lu_source_code_item!(),
            ));
        TyOfBuiltin {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for TyOfBuiltin {
    fn name(&self) -> &str {
        "type_of"
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        None
    }

    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
        let mut l_scope = scope.lock();
        let val = self.expect_arg(&mut l_scope, TO_GET_TY_OF_ARG).clone();
        Ok(Value::String(val.get_ty().to_string()))
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*TY_OF_BUILTIN_ATTRS
    }
}
