use grammar::Grammar;

#[test]
fn custom_spaces() {
    let mut grammar: Grammar<i32> = Grammar::new_with_ws("\\*");    // Replace whitespace with a single '*'.
    grammar.rule("test-a", "_");
    grammar.rule("test-b", " ");
    grammar.rule("test-c", "monkey monkey_monkey");

    let compiled = grammar.compile().unwrap();
    
    assert!(compiled.scan("test-a", "").is_err());
    assert!(compiled.scan("test-a", "***").is_ok());
    assert!(compiled.scan("test-b", "").is_ok());
    assert!(compiled.scan("test-b", "***").is_ok());
    assert!(compiled.scan("test-c", "monkey*****monkey*************monkey").is_ok());
    assert!(compiled.scan("test-c", "monkeymonkey*monkey").is_ok());
    assert!(compiled.scan("test-c", "monkey*monkeymonkey").is_err());
}