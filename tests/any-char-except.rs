use grammar::Grammar;

#[test]
fn any_char_except() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", "[^ABC💝]");
    grammar.rule("test-b", "[^ABC💝]*");
    
    assert!(grammar.scan("test-a", "").is_err());
    assert!(grammar.scan("test-a", "a").is_ok());
    assert!(grammar.scan("test-a", "A").is_err());
    assert!(grammar.scan("test-a", "💝").is_err());
    assert!(grammar.scan("test-b", "").is_ok());
    assert!(grammar.scan("test-b", "banana is love!").is_ok());
    assert!(grammar.scan("test-b", "BANANA IS LOVE!").is_err());
    assert!(grammar.scan("test-b", "banana is 💝!").is_err());
}