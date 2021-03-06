use std::{ops::Deref, rc::Rc};

use crate::cmd_prelude::*;
use lu_interpreter_structs::{ModPath, SyScope};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct ArrayPushCmd {
    sign: Signature,
}

const ARRAY_ARG_NAME: &str = "array";
const VALUES_ARG_NAME: &str = "to_push";
static PUSH_CMD_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl ArrayPushCmd {
    pub fn new() -> Self {
        let mut sign_builder = SignatureBuilder::default();
        let array_arg_ty = ArgSignature::req(
            ARRAY_ARG_NAME.into(),
            ValueType::new_array(ValueType::Generic("T".to_string()), lu_source_code_item!()),
            lu_source_code_item!(-1),
        );
        sign_builder
            .decl(lu_source_code_item!())
            .args(vec![array_arg_ty.clone()])
            .var_arg(ArgSignature::req(
                VALUES_ARG_NAME.to_string(),
                ValueType::Generic("T".to_string()),
                lu_source_code_item!(-1),
            ))
            .in_arg(ArgSignature::void(lu_source_code_item!()))
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

    fn signature(&self) -> &Signature {
        &self.sign
    }

    fn signature_item(&self) -> SourceCodeItem {
        lu_source_code_item!()
    }

    fn parent_module(&self) -> Option<&ModPath> {
        Some(&super::ARRAY_MOD_PATH)
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*PUSH_CMD_ATTRS
    }

    fn do_run_cmd(&self, scope: &mut SyScope) -> LuResult<Value> {
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
}
