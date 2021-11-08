#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use fs_extra::dir::{copy, CopyOptions};
    use lu_test_support::test_prelude::*;
    use lu_test_support::{make_test_interpreter_in_playground, Playground};
    use lu_text_util::SourceCode;
    const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");
    // const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    #[test]
    fn plugin_is_loaded() {
        let playground = Playground::new().permanent();

        // crates/lu_interpreter/test_data/resolve/example_playground/plugins/plugin1/plugin1_f1.lu
        let example_playground: PathBuf = [MANIFEST, "test_data/resolve/example_playground/"]
            .iter()
            .collect();
        let mut cp_opts = CopyOptions::new();
        cp_opts.copy_inside = true;
        cp_opts.content_only = true;
        copy(example_playground, playground.root(), &cp_opts).expect("Must work");

        let (global_frame, itprt_cfg) = make_test_interpreter_in_playground(playground);
        let eval_result = Interpreter::eval_for_tests(
            SourceCode::new_text(
                r#"
                use plugin1:plugin1_f1.lu
                plugin1_f1
            "#
                .to_string(),
            ),
            global_frame,
            &itprt_cfg,
        );
        assert!(eval_result.is_ok(), "{:?}", eval_result);
    }

    #[test]
    fn functions_are_exported_from_module() {
        let playground = Playground::new().permanent();
        playground.make_file(
            "other_file.lu",
            br#"
            fn greet
                ret "Hi from other file"
            end
            "#,
        );
        let f_path = playground.make_file(
            "first_file.lu",
            br#"
            use ./other_file.lu
            greet
            "#,
        );

        let (global_frame, itprt_cfg) = make_test_interpreter_in_playground(playground);
        let eval_result = Interpreter::eval_for_tests(
            SourceCode::new_file(f_path).unwrap(),
            global_frame,
            &itprt_cfg,
        );
        assert!(eval_result.is_ok(), "{:?}", eval_result);
    }

    #[test]
    fn structs_are_exported_from_module() {
        let playground = Playground::new().permanent();
        playground.make_file(
            "other_file.lu",
            br#"
            struct MyStruct{value:num}
            "#,
        );
        let f_path = playground.make_file(
            "first_file.lu",
            br#"
            use ./other_file.lu
            let x = MyStruct { value: 1 }
            $x
            "#,
        );

        let (global_frame, itprt_cfg) = make_test_interpreter_in_playground(playground);
        let eval_result = Interpreter::eval_for_tests(
            SourceCode::new_file(f_path).unwrap(),
            global_frame,
            &itprt_cfg,
        );
        assert!(eval_result.is_ok(), "{:?}", eval_result);
        assert_eq!(
            eval_result.unwrap(),
            Value::new_strct(
                "MyStruct".to_string(),
                vec![("value".to_string(), Value::Number(1.0.into()))]
            )
        );
    }
}
