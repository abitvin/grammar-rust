use grammar::Grammar;

#[test]
fn any_char() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", ".");
    grammar.rule("test-b", ".?");
    grammar.rule("test-c", ".+");
    grammar.rule("test-d", "\\.");

    let compiled = grammar.compile().unwrap();

    assert!(compiled.scan("test-a", "").is_err());
    assert!(compiled.scan("test-a", "A").is_ok());
    assert!(compiled.scan("test-a", "💝").is_ok());
    assert!(compiled.scan("test-a", "💝💝").is_err());
    assert!(compiled.scan("test-b", "").is_ok());
    assert!(compiled.scan("test-b", "💝").is_ok());
    assert!(compiled.scan("test-b", "💝💝").is_err());
    assert!(compiled.scan("test-c", "").is_err());
    assert!(compiled.scan("test-c", "💝").is_ok());
    assert!(compiled.scan("test-c", "💝💝").is_ok());
    assert!(compiled.scan("test-d", "A").is_err());
    assert!(compiled.scan("test-d", ".").is_ok());
}