extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn grammer_any_char_except()
{
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", "[^ABC💝]", None);
    grammer.add("test-b", "[^ABC💝]*", None);
    
    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "a").is_ok());
    assert!(grammer.scan("test-a", "A").is_err());
    assert!(grammer.scan("test-a", "💝").is_err());
    assert!(grammer.scan("test-b", "").is_ok());
    assert!(grammer.scan("test-b", "banana is love!").is_ok());
    assert!(grammer.scan("test-b", "BANANA IS LOVE!").is_err());
    assert!(grammer.scan("test-b", "banana is 💝!").is_err());
}