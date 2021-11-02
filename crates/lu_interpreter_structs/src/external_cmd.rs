use lu_error::lu_source_code_item;
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARGS_DEF_NAME};

use crate::{ArgSignature, Signature, ValueType};

pub fn external_cmd_signature() -> Signature {
    let lu_item = lu_source_code_item!();
    Signature::new(
        Vec::new(),
        Some(ArgSignature::new(
            VAR_ARGS_DEF_NAME.into(),
            ValueType::Any,
            lu_item.clone(),
        )),
        Vec::new(),
        ArgSignature::new(IN_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
        ArgSignature::new(RET_ARG_NAME.into(), ValueType::Any, lu_item.clone()),
        lu_item,
    )
}
