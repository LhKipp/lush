use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct TestPrintCmd {
    sign: Signature,
}

impl TestPrintCmd {
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
            .in_type(ArgSignature::void(print_decl.clone().into()))
            .ret_type(ArgSignature::void(print_decl.into()));
        TestPrintCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for TestPrintCmd {
    fn signature_item(&self) -> lu_error::SourceCodeItem {
        lu_source_code_item!()
    }

    fn signature(&self) -> &lu_interpreter::Signature {
        todo!()
    }

    fn name(&self) -> &str {
        "tprint"
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let mut l_scope = state.scope.lock();
        let args = self.expect_args(&l_scope).clone();
        let global_f = l_scope.global_mut_frame();

        let var = "t_printed".to_string();
        if let Some(test_print_vars) = global_f.get_mut(&var) {
            let vals = test_print_vars.val.expect_array();
            let len = args.len();
            vals.extend((0..len).map(move |i| args[i].clone()))
        } else {
            debug!("Inserted t_printed");
            global_f.insert(
                var.clone(),
                Variable::new(var.clone(), Value::Array(args.clone()), None),
            );
        }
        Ok(Value::Nil)
    }
}
