extern crate grammer;
use grammer::Grammer;

#[test]
fn char_in() {
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", "[a-z]", None);
    grammer.add("test-b", "[😀-🙏]", None);         // Emoticons (Emoji) U+1F600 — U+1F64F
    grammer.add("test-c", "[a-zA-Z0-9]+", None);
    
    assert!(grammer.scan("test-a", "").is_err());
    assert!(grammer.scan("test-a", "x").is_ok());
    assert!(grammer.scan("test-a", "A").is_err());
    assert!(grammer.scan("test-b", "☺").is_err());  // Alhough a smiley (emoji), this char (U+263A) is not in the range we given. 
    assert!(grammer.scan("test-b", "😍").is_ok());
    assert!(grammer.scan("test-b", "😷").is_ok());
    assert!(grammer.scan("test-c", "Banana304").is_ok());
    assert!(grammer.scan("test-c", "Monkey80085").is_ok());
}