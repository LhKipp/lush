use std::{ops::Deref, rc::Rc};

use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct ArrayPushCmd {
    sign: Signature,
}

const ARRAY_ARG_NAME: &str = "array";
const VALUES_ARG_NAME: &str = "to_push";

impl ArrayPushCmd {
    pub fn new() -> Self {
        let push_decl = lu_source_code_item!();
        let mut sign_builder = SignatureBuilder::default();
        let array_arg_ty = ArgSignature::new(
            ARRAY_ARG_NAME.into(),
            ValueType::new_array(ValueType::Generic("T".to_string()), push_decl.clone()),
            push_decl.clone().into(),
        );
        sign_builder
            .decl(push_decl.clone())
            .args(vec![array_arg_ty])
            .var_arg(ArgSignature::new(
                format!("...{}", VALUES_ARG_NAME),
                ValueType::Generic("T".to_string()),
                push_decl.clone().into(),
            ))
            .in_arg(ArgSignature::void(push_decl.clone().into()))
            .ret_arg(ArgSignature::void(push_decl.clone().into()));

        ArrayPushCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for ArrayPushCmd {
    fn name(&self) -> &str {
        "push"
    }

    fn do_run(&self, _: &[EvalArg], state: &mut Evaluator) -> LuResult<Value> {
        let mut l_scope = state.scope.lock();
        let values_to_push = self
            .expect_arg(&l_scope, VALUES_ARG_NAME)
            .as_array()
            .unwrap()
            .deref()
            .clone();

        if let Value::Array(array) = self.expect_mut_arg(&mut l_scope, ARRAY_ARG_NAME) {
            let array = Rc::make_mut(array);
            array.extend(values_to_push)
        } else {
            unreachable!("ARRAY_ARG_NAME is of array type");
        }

        Ok(Value::Nil)
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }
}
