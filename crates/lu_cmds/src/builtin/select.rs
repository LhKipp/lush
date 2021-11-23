use lu_interpreter_structs::special_cmds::SELECT_DEF_STRCT_DECL_ARG_NAME;

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
                ValueType::String,
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
            .flags(vec![FlagSignature::opt(
                Some("gen_struct_name".into()),
                Some('n'),
                ValueType::String,
                lu_source_code_item!(-4),
            )])
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
        let l_scope = scope.lock();
        // let val = self.expect_args(COL_NAMES, &l_scope).clone();
        let table = self.expect_in(&l_scope).as_array().unwrap().clone();

        let gen_strct_decl = self
            .expect_arg(&l_scope, SELECT_DEF_STRCT_DECL_ARG_NAME)
            .as_strct_decl()
            .expect("Arg is always passed and is strct decl");
        let l_gen_strct_decl = gen_strct_decl.read();

        let selected_cols: Vec<_> = table
            .iter()
            .map(|row| {
                let (_, row) = row.as_strct().expect("Always strct");

                let selected_vals: Vec<_> = l_gen_strct_decl
                    .fields
                    .iter()
                    .map(|field_decl| {
                        row.iter()
                            .find_map(|(col_name, col_val)| {
                                if *col_name == field_decl.name {
                                    Some((field_decl.name.clone(), col_val.clone()))
                                } else {
                                    None
                                }
                            })
                            .expect("Always found")
                    })
                    .collect();

                Value::new_strct(l_gen_strct_decl.name.clone(), selected_vals)
            })
            .collect();

        Ok(Value::new_array(selected_cols))
    }

    fn attributes(&self) -> &[CmdAttribute] {
        &*SELECT_BUILTIN_ATTRS
    }
}
