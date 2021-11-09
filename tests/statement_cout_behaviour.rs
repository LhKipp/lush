use lu_test_support::*;

#[test]
fn piped_cmds_stmt_do_not_print() {
    let tmp_file_path = make_tmp_file(b"echo hi | cat | cat | cat");
    let (code, cout, cerr) =
        binary::run_binary(&[&tmp_file_path.path().to_string_lossy().to_string()]);
    assert_eq!(code, 0);
    assert_eq!(cerr, "");
    assert_eq!(cout, "hi\n");
}

#[test]
fn internal_fn_always_writes_to_stdout() {
    let tmp_file_path = make_tmp_file(
        r#"
        fn print_and_ret_num
            echo printed
            ret 1
        end
        print_and_ret_num | cat

        let x = print_and_ret_num
        "#
        .as_bytes(),
    );
    let (code, cout, cerr) =
        binary::run_binary(&[&tmp_file_path.path().to_string_lossy().to_string()]);
    assert_eq!(code, 0);
    assert_eq!(cerr, "");
    assert_eq!(cout, "printed\n1\nprinted\n");
}

#[test]
fn external_cmd_not_printing_when_is_rhs_of_let() {
    let tmp_file_path = make_tmp_file(
        r#"
        let x = echo hi
        "#
        .as_bytes(),
    );
    let (code, cout, cerr) =
        binary::run_binary(&[&tmp_file_path.path().to_string_lossy().to_string()]);
    assert_eq!(code, 0);
    assert_eq!(cerr, "");
    assert_eq!(cout, "");
}
