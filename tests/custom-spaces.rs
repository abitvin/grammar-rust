extern crate grammer;
use grammer::Grammer;

#[test]
fn custom_spaces()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    
    grammer.ws("\\*");  // Replace whitespace with a single '*'.

    grammer.add("test-a", "_", None);
    grammer.add("test-b", " ", None);
    grammer.add("test-c", "monkey monkey_monkey", None);
    
    assert!(grammer.scan("test-a", "", &mut false).is_err());
    assert!(grammer.scan("test-a", "***", &mut false).is_ok());
    assert!(grammer.scan("test-b", "", &mut false).is_ok());
    assert!(grammer.scan("test-b", "***", &mut false).is_ok());
    assert!(grammer.scan("test-c", "monkey*****monkey*************monkey", &mut false).is_ok());
    assert!(grammer.scan("test-c", "monkeymonkey*monkey", &mut false).is_ok());
    assert!(grammer.scan("test-c", "monkey*monkeymonkey", &mut false).is_err());
}