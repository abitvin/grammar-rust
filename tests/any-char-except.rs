use grammar::Grammar;

#[test]
fn any_char_except() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", "[^ABC💝]");
    grammar.rule("test-b", "[^ABC💝]*");

    let compiled = grammar.compile().unwrap();
    
    assert!(compiled.scan("test-a", "").is_err());
    assert!(compiled.scan("test-a", "a").is_ok());
    assert!(compiled.scan("test-a", "A").is_err());
    assert!(compiled.scan("test-a", "💝").is_err());
    assert!(compiled.scan("test-b", "").is_ok());
    assert!(compiled.scan("test-b", "banana is love!").is_ok());
    assert!(compiled.scan("test-b", "BANANA IS LOVE!").is_err());
    assert!(compiled.scan("test-b", "banana is 💝!").is_err());
}