extern crate grammer;
use grammer::Grammer;

#[test]
fn eof()
{
    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("new-line", "\r?\n", None);      // TODO Fix id naming bug, rename this id to something like "aaaaaaa" will make the test successfull.
    grammer.add("line", "line(<new-line>|$)", None);
    grammer.add("root", "<line>*", None);

    assert!(grammer.scan("root", "", &mut false).is_ok());
    assert!(grammer.scan("root", "line", &mut false).is_ok());
    assert!(grammer.scan("root", "line\n", &mut false).is_ok());
    assert!(grammer.scan("root", "line\nline", &mut false).is_ok());
    assert!(grammer.scan("root", "line\r\nline\n", &mut false).is_ok());
}