extern crate grammar;
use grammar::Grammar;

#[test]
fn eof() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("new-line", "\r?\n", None);      // TODO Fix id naming bug, rename this id to something like "aaaaaaa" will make the test successfull.
    grammar.add("line", "line(<new-line>|$)", None);
    grammar.add("root", "<line>*", None);

    assert!(grammar.scan("root", "").is_ok());
    assert!(grammar.scan("root", "line").is_ok());
    assert!(grammar.scan("root", "line\n").is_ok());
    assert!(grammar.scan("root", "line\nline").is_ok());
    assert!(grammar.scan("root", "line\r\nline\n").is_ok());
}