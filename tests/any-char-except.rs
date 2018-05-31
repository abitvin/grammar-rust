extern crate grammar;
use grammar::Grammar;

#[test]
fn any_char_except() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("test-a", "[^ABC💝]", None);
    grammar.add("test-b", "[^ABC💝]*", None);
    
    assert!(grammar.scan("test-a", "").is_err());
    assert!(grammar.scan("test-a", "a").is_ok());
    assert!(grammar.scan("test-a", "A").is_err());
    assert!(grammar.scan("test-a", "💝").is_err());
    assert!(grammar.scan("test-b", "").is_ok());
    assert!(grammar.scan("test-b", "banana is love!").is_ok());
    assert!(grammar.scan("test-b", "BANANA IS LOVE!").is_err());
    assert!(grammar.scan("test-b", "banana is 💝!").is_err());
}