use grammar::Grammar;

#[test]
fn any_of() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("a", "a");
    grammar.rule("abc", "(<a>|b|c+)");
    grammar.rule("test-a", "<abc>");
    grammar.rule("test-b", "XXX<abc>");
    grammar.rule("test-c", "<abc>YYY");
    grammar.rule("test-d", "XXX<abc>YYY");

    assert!(grammar.scan("test-a", "a").is_ok());
    assert!(grammar.scan("test-a", "aa").is_err());
    assert!(grammar.scan("test-a", "b").is_ok());
    assert!(grammar.scan("test-a", "bb").is_err());
    assert!(grammar.scan("test-a", "x").is_err());
    assert!(grammar.scan("test-a", "c").is_ok());
    assert!(grammar.scan("test-a", "cc").is_ok());

    assert!(grammar.scan("test-b", "XXXa").is_ok());
    assert!(grammar.scan("test-b", "XXXaa").is_err());
    assert!(grammar.scan("test-b", "XXXb").is_ok());
    assert!(grammar.scan("test-b", "XXXbb").is_err());
    assert!(grammar.scan("test-b", "XXXx").is_err());
    assert!(grammar.scan("test-b", "XXXc").is_ok());
    assert!(grammar.scan("test-b", "XXXcc").is_ok());

    assert!(grammar.scan("test-c", "aYYY").is_ok());
    assert!(grammar.scan("test-c", "aaYYY").is_err());
    assert!(grammar.scan("test-c", "bYYY").is_ok());
    assert!(grammar.scan("test-c", "bbYYY").is_err());
    assert!(grammar.scan("test-c", "xYYY").is_err());
    assert!(grammar.scan("test-c", "cYYY").is_ok());
    assert!(grammar.scan("test-c", "ccYYY").is_ok());
    
    assert!(grammar.scan("test-d", "XXXaYYY").is_ok());
    assert!(grammar.scan("test-d", "XXXaaYYY").is_err());
    assert!(grammar.scan("test-d", "XXXbYYY").is_ok());
    assert!(grammar.scan("test-d", "XXXbbYYY").is_err());
    assert!(grammar.scan("test-d", "XXXxYYY").is_err());
    assert!(grammar.scan("test-d", "XXXcYYY").is_ok());
    assert!(grammar.scan("test-d", "XXXccYYY").is_ok());
}