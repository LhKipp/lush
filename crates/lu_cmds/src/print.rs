use lu_interpreter_structs::ModPath;

use crate::cmd_prelude::*;

#[derive(Debug, Clone)]
pub struct PrintCmd {
    sign: Signature,
}

const TO_PRINT_ARG_NAME: &str = "to_print";
static PRINT_CMD_ATTRS: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![CmdAttribute::new(Pure, lu_source_code_item!())]);

impl PrintCmd {
    pub fn new() -> Self {
        let print_decl = lu_source_code_item!();
        let mut sign_builder = SignatureBuilder::default();
        sign_builder
            .decl(print_decl.clone())
            .var_arg(ArgSignature::req(
                TO_PRINT_ARG_NAME.to_string(),
                ValueType::Any,
                print_decl.clone().into(),
            ))
            .in_arg(ArgSignature::void(print_decl.clone().into()))
            .ret_arg(ArgSignature::ret(ValueType::Nil, print_decl.into()));
        PrintCmd {
            sign: sign_builder.build().unwrap(),
        }
    }
}

impl Command for PrintCmd {
    fn name(&self) -> &str {
        "print"
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
        let args = self.take_var_arg(&mut l_scope, TO_PRINT_ARG_NAME).clone();

        if let Some(redir_to) = l_scope.find_var_mut(REDIR0) {
            let new_val = match std::mem::replace(&mut redir_to.val, Value::Nil) {
                Value::Array(arr) => {
                    let mut inner_arr = (*arr).clone();
                    inner_arr.extend(args);
                    Value::new_array(inner_arr)
                }
                Value::String(_) => todo!(),
                Value::Command(_) => todo!(),
                _ => unreachable!(),
            };
            redir_to.val = new_val;
        } else {
            // Simple print
            for arg in args {
                print!("{}", arg.to_string())
            }
            println!("");
        }
        Ok(Value::Nil)
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*PRINT_CMD_ATTRS
    }
}

const REDIR0: &str = "REDIR0";
