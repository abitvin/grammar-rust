extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn eof()
{
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("new-line", "\r?\n", None);      // TODO Fix id naming bug, rename this id to something like "aaaaaaa" will make the test successfull.
    grammer.add("line", "line(<new-line>|$)", None);
    grammer.add("root", "<line>*", None);

    assert!(grammer.scan("root", "").is_ok());
    assert!(grammer.scan("root", "line").is_ok());
    assert!(grammer.scan("root", "line\n").is_ok());
    assert!(grammer.scan("root", "line\nline").is_ok());
    assert!(grammer.scan("root", "line\r\nline\n").is_ok());
}