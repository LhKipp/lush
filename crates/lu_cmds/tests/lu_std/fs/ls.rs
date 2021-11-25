use lu_test_support::test_prelude::*;

#[test]
fn ls_gives_back_table() {
    let playground = Playground::new().permanent();
    playground.make_file("file.txt", b" ");
    playground.make_dirs("dir_a");

    let (global_frame, itprt_cfg) = make_test_interpreter_in_playground(playground);
    let eval_result = Interpreter::eval_for_tests(
        r#"
        use std:fs
        ls
        "#
        .to_string()
        .into(),
        global_frame,
        &itprt_cfg,
    );
    assert!(eval_result.is_ok(), "{:?}", eval_result);
    let formatted = format!("{:#?}", eval_result.unwrap());
    assert_eq!(
        formatted.trim(),
        r#"
 name     | type      | size 
 dir_a    | Directory | 40 
 file.txt | File      | 1 
 plugins  | Directory | 40 
"#
        .trim()
    );
}
