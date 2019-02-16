use grammar::Grammar;

#[test]
fn any_char() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("test-a", ".", None);
    grammar.add("test-b", ".?", None);
    grammar.add("test-c", ".+", None);
    grammar.add("test-d", "\\.", None);

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