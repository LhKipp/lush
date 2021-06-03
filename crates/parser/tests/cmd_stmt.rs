use parser::parse;

#[test]
fn parses_cmd_name() {
    pretty_env_logger::init();
    let events = parse(
        r#"
    fn ls ()
        echo "hi"
    end
    "#,
    );
    assert_eq!(events.len(), 29);
}
