use grammar::Grammar;

#[test]
fn eof() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("new-line", "\r?\n");
    grammar.rule("line", "line(<new-line>|$)");
    grammar.rule("root", "<line>*");

    assert!(grammar.scan("root", "").is_ok());
    assert!(grammar.scan("root", "line").is_ok());
    assert!(grammar.scan("root", "line\n").is_ok());
    assert!(grammar.scan("root", "line\nline").is_ok());
    assert!(grammar.scan("root", "line\r\nline\n").is_ok());
}