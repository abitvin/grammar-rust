extern crate grammer;
use grammer::Grammer;

#[test]
fn whitespace() {
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", "_", None);   // At least one whitespace
    grammer.add("test-b", " ", None);   // None or many whitespaces
    grammer.add("test-c", "monkey monkey_monkey", None);

    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "   ").is_ok());
    assert!(grammer.scan("test-b", "").is_ok());
    assert!(grammer.scan("test-b", "   ").is_ok());
    assert!(grammer.scan("test-c", "monkey     monkey                      monkey").is_ok());
    assert!(grammer.scan("test-c", "monkeymonkey monkey").is_ok());
    assert!(grammer.scan("test-c", "monkey monkeymonkey").is_err());
}