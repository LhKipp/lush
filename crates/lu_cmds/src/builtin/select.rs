use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct SelectBuiltin {
    sign: Signature,
}

const COL_NAMES: &str = "col_names";
static SELECT_BUILTIN_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl SelectBuiltin {
    pub fn new() -> Self {
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(lu_source_code_item!())
            .var_arg(ArgSignature::req(
                COL_NAMES.to_string(),
                ValueType::Any,
                lu_source_code_item!(-3).into(),
            ))
            .in_arg(ArgSignature::req(
                "table".into(),
                ValueType::Array {
                    inner_ty: Box::new(ValueType::Any),
                    inner_ty_decl: lu_source_code_item!(),
                },
                lu_source_code_item!(),
            ))
            .ret_arg(ArgSignature::req(
                "projected_table".into(),
                ValueType::Any,
                lu_source_code_item!(),
            ));
        SelectBuiltin {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for SelectBuiltin {
    fn name(&self) -> &str {
        "select"
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
        let val = self.expect_arg(&mut l_scope, COL_NAMES).clone();
        Ok(Value::String(val.get_ty().to_string()))
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*SELECT_BUILTIN_ATTRS
    }
}
