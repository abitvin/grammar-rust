use grammar::Grammar;

#[test]
fn any_char() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", ".");
    grammar.rule("test-b", ".?");
    grammar.rule("test-c", ".+");
    grammar.rule("test-d", "\\.");

    assert!(grammar.scan("test-a", "").is_err());
    assert!(grammar.scan("test-a", "A").is_ok());
    assert!(grammar.scan("test-a", "💝").is_ok());
    assert!(grammar.scan("test-a", "💝💝").is_err());
    assert!(grammar.scan("test-b", "").is_ok());
    assert!(grammar.scan("test-b", "💝").is_ok());
    assert!(grammar.scan("test-b", "💝💝").is_err());
    assert!(grammar.scan("test-c", "").is_err());
    assert!(grammar.scan("test-c", "💝").is_ok());
    assert!(grammar.scan("test-c", "💝💝").is_ok());
    assert!(grammar.scan("test-d", "A").is_err());
    assert!(grammar.scan("test-d", ".").is_ok());
}