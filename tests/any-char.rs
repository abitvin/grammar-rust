extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn grammer_any_char()
{
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", ".", None);
    grammer.add("test-b", ".?", None);
    grammer.add("test-c", ".+", None);
    grammer.add("test-d", "\\.", None);
    
    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "A").is_ok());
    assert!(grammer.scan("test-a", "💝").is_ok());
    assert!(grammer.scan("test-a", "💝💝").is_err());
    assert!(grammer.scan("test-b", "").is_ok());
    assert!(grammer.scan("test-b", "💝").is_ok());
    assert!(grammer.scan("test-b", "💝💝").is_err());
    assert!(grammer.scan("test-c", "").is_err());
    assert!(grammer.scan("test-c", "💝").is_ok());
    assert!(grammer.scan("test-c", "💝💝").is_ok());
    assert!(grammer.scan("test-d", "A").is_err());
    assert!(grammer.scan("test-d", ".").is_ok());
}