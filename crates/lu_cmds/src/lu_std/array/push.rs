use std::{ops::Deref, rc::Rc, sync::Arc};

use crate::cmd_prelude::*;
use lu_interpreter_structs::{ModPath, Scope};
use parking_lot::Mutex;

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
            .args(vec![array_arg_ty.clone()])
            .var_arg(ArgSignature::new(
                VALUES_ARG_NAME.to_string(),
                ValueType::Generic("T".to_string()),
                push_decl.clone().into(),
            ))
            .in_arg(ArgSignature::void(push_decl.clone().into()))
            .ret_arg(array_arg_ty);

        ArrayPushCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for ArrayPushCmd {
    fn name(&self) -> &str {
        "push"
    }

    fn do_run_cmd(&self, scope: &mut Arc<Mutex<Scope<Variable>>>) -> LuResult<Value> {
        let mut l_scope = scope.lock();
        let values_to_push = self
            .expect_arg(&l_scope, VALUES_ARG_NAME)
            .as_array()
            .unwrap()
            .deref()
            .clone();

        if let Value::Array(array) = self.expect_mut_arg(&mut l_scope, ARRAY_ARG_NAME) {
            let array_mut = Rc::make_mut(array);
            array_mut.extend(values_to_push);
            Ok(Value::Array(array.clone()))
        } else {
            unreachable!("ARRAY_ARG_NAME is of array type");
        }
    }

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        Some(&super::ARRAY_MOD_PATH)
    }
}
