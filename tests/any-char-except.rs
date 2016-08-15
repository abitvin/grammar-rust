extern crate grammer;
use grammer::Grammer;

#[test]
fn grammer_any_char_except()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("test-a", "[^ABC💝]", None);
    grammer.add("test-b", "[^ABC💝]*", None);
    
    assert!(grammer.scan("test-a", "", &mut false).is_err());
    assert!(grammer.scan("test-a", "a", &mut false).is_ok());
    assert!(grammer.scan("test-a", "A", &mut false).is_err());
    assert!(grammer.scan("test-a", "💝", &mut false).is_err());
    assert!(grammer.scan("test-b", "", &mut false).is_ok());
    assert!(grammer.scan("test-b", "banana is love!", &mut false).is_ok());
    assert!(grammer.scan("test-b", "BANANA IS LOVE!", &mut false).is_err());
    assert!(grammer.scan("test-b", "banana is 💝!", &mut false).is_err());
}