#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use fs_extra::dir::{copy, CopyOptions};
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

        let mut itprtr = make_test_interpreter_in_playground(playground);
        let eval_result = itprtr.eval(SourceCode::new_text(
            r#"
                use plugin1:plugin1_f1.lu
                plugin1_f1
            "#
            .to_string(),
        ));
        assert!(eval_result.is_ok(), "{:?}", eval_result);
    }
}
