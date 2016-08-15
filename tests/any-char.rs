extern crate grammer;
use grammer::Grammer;

#[test]
fn grammer_any_char()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("test-a", ".", None);
    grammer.add("test-b", ".?", None);
    grammer.add("test-c", ".+", None);
    grammer.add("test-d", "\\.", None);
    
    assert!(grammer.scan("test-a", "", &mut false).is_err());
    assert!(grammer.scan("test-a", "A", &mut false).is_ok());
    assert!(grammer.scan("test-a", "💝", &mut false).is_ok());
    assert!(grammer.scan("test-a", "💝💝", &mut false).is_err());
    assert!(grammer.scan("test-b", "", &mut false).is_ok());
    assert!(grammer.scan("test-b", "💝", &mut false).is_ok());
    assert!(grammer.scan("test-b", "💝💝", &mut false).is_err());
    assert!(grammer.scan("test-c", "", &mut false).is_err());
    assert!(grammer.scan("test-c", "💝", &mut false).is_ok());
    assert!(grammer.scan("test-c", "💝💝", &mut false).is_ok());
    assert!(grammer.scan("test-d", "A", &mut false).is_err());
    assert!(grammer.scan("test-d", ".", &mut false).is_ok());
}