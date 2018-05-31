extern crate grammar;
use grammar::Grammar;

#[test]
fn custom_spaces() {
    let mut grammar: Grammar<i32> = Grammar::new_with_ws("\\*");    // Replace whitespace with a single '*'.
    
    grammar.add("test-a", "_", None);
    grammar.add("test-b", " ", None);
    grammar.add("test-c", "monkey monkey_monkey", None);
    
    assert!(grammar.scan("test-a", "").is_err());
    assert!(grammar.scan("test-a", "***").is_ok());
    assert!(grammar.scan("test-b", "").is_ok());
    assert!(grammar.scan("test-b", "***").is_ok());
    assert!(grammar.scan("test-c", "monkey*****monkey*************monkey").is_ok());
    assert!(grammar.scan("test-c", "monkeymonkey*monkey").is_ok());
    assert!(grammar.scan("test-c", "monkey*monkeymonkey").is_err());
}