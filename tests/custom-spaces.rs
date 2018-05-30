extern crate grammer;
use grammer::Grammer;

#[test]
fn custom_spaces() {
    let mut grammer: Grammer<i32> = Grammer::new_with_ws("\\*");    // Replace whitespace with a single '*'.
    
    grammer.add("test-a", "_", None);
    grammer.add("test-b", " ", None);
    grammer.add("test-c", "monkey monkey_monkey", None);
    
    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "***").is_ok());
    assert!(grammer.scan("test-b", "").is_ok());
    assert!(grammer.scan("test-b", "***").is_ok());
    assert!(grammer.scan("test-c", "monkey*****monkey*************monkey").is_ok());
    assert!(grammer.scan("test-c", "monkeymonkey*monkey").is_ok());
    assert!(grammer.scan("test-c", "monkey*monkeymonkey").is_err());
}