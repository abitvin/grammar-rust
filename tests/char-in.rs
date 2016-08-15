extern crate grammer;
use grammer::Grammer;

#[test]
fn char_in()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("test-a", "[a-z]", None);
    grammer.add("test-b", "[😀-🙏]", None);         // Emoticons (Emoji) U+1F600 — U+1F64F
    grammer.add("test-c", "[a-zA-Z0-9]+", None);
    
    assert!(grammer.scan("test-a", "", &mut false).is_err());
    assert!(grammer.scan("test-a", "x", &mut false).is_ok());
    assert!(grammer.scan("test-a", "A", &mut false).is_err());
    assert!(grammer.scan("test-b", "☺", &mut false).is_err());  // Alhough a smiley (emoji), this char (U+263A) is not in the range we given. 
    assert!(grammer.scan("test-b", "😍", &mut false).is_ok());
    assert!(grammer.scan("test-b", "😷", &mut false).is_ok());
    assert!(grammer.scan("test-c", "Banana304", &mut false).is_ok());
    assert!(grammer.scan("test-c", "Monkey80085", &mut false).is_ok());
}