use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct PrintCmd {
    sign: Signature,
}

impl PrintCmd {
    pub fn new() -> Self {
        let print_decl = lu_source_code_item!();
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(print_decl.clone())
            .var_arg(ArgSignature::new(
                "to_print".into(),
                ValueType::Any,
                print_decl.clone().into(),
            ))
            .in_arg(ArgSignature::void(print_decl.clone().into()))
            .ret_arg(ArgSignature::ret(
                ValueType::new_array(ValueType::Any, print_decl.clone()),
                print_decl.into(),
            ));
        PrintCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for PrintCmd {
    fn name(&self) -> &str {
        "print"
    }

    fn do_run_cmd(&self, scope: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<Value> {
        let l_scope = scope.lock();
        let args = self.expect_args(&l_scope);
        Ok(Value::Array(args.clone()))
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }
}
