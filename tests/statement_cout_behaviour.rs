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
