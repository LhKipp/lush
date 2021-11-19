use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct IsSetBuiltin {
    sign: Signature,
}

const VAR_NAME_TO_LOOKUP: &str = "var_name";
static IS_SET_BUILTIN_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl IsSetBuiltin {
    pub fn new() -> Self {
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(lu_source_code_item!())
            .args(vec![ArgSignature::new(
                VAR_NAME_TO_LOOKUP.to_string(),
                ValueType::String,
                lu_source_code_item!(-3).into(),
            )])
            .ret_arg(ArgSignature::new(
                "value_exists".into(),
                ValueType::Bool,
                lu_source_code_item!(),
            ));
        IsSetBuiltin {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for IsSetBuiltin {
    fn name(&self) -> &str {
        "is_set"
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
        let l_scope = scope.lock();
        let var_to_find = self
            .expect_arg(&l_scope, VAR_NAME_TO_LOOKUP)
            .coerce_to_string()
            .unwrap();

        let var_to_find_exists = l_scope.find_var(var_to_find).is_some();

        Ok(Value::Bool(var_to_find_exists))
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*IS_SET_BUILTIN_ATTRS
    }
}
