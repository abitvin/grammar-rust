extern crate grammer;
use grammer::Grammer;

#[test]
fn grammer_any_of()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("a", "a", None);
    grammer.add("abc", "(<a>|b|c+)", None);
    grammer.add("test-a", "<abc>", None);
    grammer.add("test-b", "XXX<abc>", None);
    grammer.add("test-c", "<abc>YYY", None);
    grammer.add("test-d", "XXX<abc>YYY", None);

    assert!(grammer.scan("test-a", "a", &mut false).is_ok());
    assert!(grammer.scan("test-a", "aa", &mut false).is_err());
    assert!(grammer.scan("test-a", "b", &mut false).is_ok());
    assert!(grammer.scan("test-a", "bb", &mut false).is_err());
    assert!(grammer.scan("test-a", "x", &mut false).is_err());
    assert!(grammer.scan("test-a", "c", &mut false).is_ok());
    assert!(grammer.scan("test-a", "cc", &mut false).is_ok());

    assert!(grammer.scan("test-b", "XXXa", &mut false).is_ok());
    assert!(grammer.scan("test-b", "XXXaa", &mut false).is_err());
    assert!(grammer.scan("test-b", "XXXb", &mut false).is_ok());
    assert!(grammer.scan("test-b", "XXXbb", &mut false).is_err());
    assert!(grammer.scan("test-b", "XXXx", &mut false).is_err());
    assert!(grammer.scan("test-b", "XXXc", &mut false).is_ok());
    assert!(grammer.scan("test-b", "XXXcc", &mut false).is_ok());

    assert!(grammer.scan("test-c", "aYYY", &mut false).is_ok());
    assert!(grammer.scan("test-c", "aaYYY", &mut false).is_err());
    assert!(grammer.scan("test-c", "bYYY", &mut false).is_ok());
    assert!(grammer.scan("test-c", "bbYYY", &mut false).is_err());
    assert!(grammer.scan("test-c", "xYYY", &mut false).is_err());
    assert!(grammer.scan("test-c", "cYYY", &mut false).is_ok());
    assert!(grammer.scan("test-c", "ccYYY", &mut false).is_ok());
    
    assert!(grammer.scan("test-d", "XXXaYYY", &mut false).is_ok());
    assert!(grammer.scan("test-d", "XXXaaYYY", &mut false).is_err());
    assert!(grammer.scan("test-d", "XXXbYYY", &mut false).is_ok());
    assert!(grammer.scan("test-d", "XXXbbYYY", &mut false).is_err());
    assert!(grammer.scan("test-d", "XXXxYYY", &mut false).is_err());
    assert!(grammer.scan("test-d", "XXXcYYY", &mut false).is_ok());
    assert!(grammer.scan("test-d", "XXXccYYY", &mut false).is_ok());
}