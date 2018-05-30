extern crate grammer;
use grammer::Grammer;

#[test]
fn any_of() {
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("a", "a", None);
    grammer.add("abc", "(<a>|b|c+)", None);
    grammer.add("test-a", "<abc>", None);
    grammer.add("test-b", "XXX<abc>", None);
    grammer.add("test-c", "<abc>YYY", None);
    grammer.add("test-d", "XXX<abc>YYY", None);

    assert!(grammer.scan("test-a", "a").is_ok());
    assert!(grammer.scan("test-a", "aa").is_err());
    assert!(grammer.scan("test-a", "b").is_ok());
    assert!(grammer.scan("test-a", "bb").is_err());
    assert!(grammer.scan("test-a", "x").is_err());
    assert!(grammer.scan("test-a", "c").is_ok());
    assert!(grammer.scan("test-a", "cc").is_ok());

    assert!(grammer.scan("test-b", "XXXa").is_ok());
    assert!(grammer.scan("test-b", "XXXaa").is_err());
    assert!(grammer.scan("test-b", "XXXb").is_ok());
    assert!(grammer.scan("test-b", "XXXbb").is_err());
    assert!(grammer.scan("test-b", "XXXx").is_err());
    assert!(grammer.scan("test-b", "XXXc").is_ok());
    assert!(grammer.scan("test-b", "XXXcc").is_ok());

    assert!(grammer.scan("test-c", "aYYY").is_ok());
    assert!(grammer.scan("test-c", "aaYYY").is_err());
    assert!(grammer.scan("test-c", "bYYY").is_ok());
    assert!(grammer.scan("test-c", "bbYYY").is_err());
    assert!(grammer.scan("test-c", "xYYY").is_err());
    assert!(grammer.scan("test-c", "cYYY").is_ok());
    assert!(grammer.scan("test-c", "ccYYY").is_ok());
    
    assert!(grammer.scan("test-d", "XXXaYYY").is_ok());
    assert!(grammer.scan("test-d", "XXXaaYYY").is_err());
    assert!(grammer.scan("test-d", "XXXbYYY").is_ok());
    assert!(grammer.scan("test-d", "XXXbbYYY").is_err());
    assert!(grammer.scan("test-d", "XXXxYYY").is_err());
    assert!(grammer.scan("test-d", "XXXcYYY").is_ok());
    assert!(grammer.scan("test-d", "XXXccYYY").is_ok());
}