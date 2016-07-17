extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn custom_spaces()
{
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", "_", None);
    grammer.add("test-b", " ", None);
    grammer.add("test-c", "monkey monkey_monkey", None);
    
    // It's better practice to add the whitespace declaration at the beginning.
    grammer.ws("\\*");    // TODO Make a more advanced whitespace rule.

    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "***").is_ok());
    assert!(grammer.scan("test-b", "").is_ok());
    assert!(grammer.scan("test-b", "***").is_ok());
    assert!(grammer.scan("test-c", "monkey*****monkey*************monkey").is_ok());
    assert!(grammer.scan("test-c", "monkeymonkey*monkey").is_ok());
    assert!(grammer.scan("test-c", "monkey*monkeymonkey").is_err());
}