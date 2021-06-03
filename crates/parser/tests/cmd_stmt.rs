use parser::parse;

#[test]
fn parses_cmd_name() {
    let events = parse(r#"fn ls () { echo "hi" }"#);
    assert_eq!(events.len(), 4);
}
