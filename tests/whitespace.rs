use grammar::Grammar;

#[test]
fn whitespace() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", "_");   // At least one whitespace
    grammar.rule("test-b", " ");   // None or many whitespaces
    grammar.rule("test-c", "monkey monkey_monkey");

    let compiled = grammar.compile().unwrap();

    assert!(compiled.scan("test-a", "").is_err());
    assert!(compiled.scan("test-a", "   ").is_ok());
    assert!(compiled.scan("test-b", "").is_ok());
    assert!(compiled.scan("test-b", "   ").is_ok());
    assert!(compiled.scan("test-c", "monkey     monkey                      monkey").is_ok());
    assert!(compiled.scan("test-c", "monkeymonkey monkey").is_ok());
    assert!(compiled.scan("test-c", "monkey monkeymonkey").is_err());
}