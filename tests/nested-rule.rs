use grammar::Grammar;

#[test]
fn nested_rule() {
    let f = |_, _: &str| 7777;

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("test-a", "<monkey>");
    grammar.rule("test-b", "<monkey><monkey><monkey>");
    grammar.rule("test-c", "<monkey>+");
    grammar.rule("test-d", "<monkey>*");
    grammar.map("monkey", "monkey", f);

    if let Ok(_) = grammar.scan("test-a", "ape") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammar.scan("test-a", "monkey") {
        assert_eq!(branches[0], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("test-b", "monkeymonkeymonkey") {
        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammar.scan("test-c", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammar.scan("test-c", "monkeymonkeymonkeymonkey") {
        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
        assert_eq!(branches[3], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("test-d", "") {
        assert_eq!(branches.len(), 0);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("test-d", "monkeymonkeymonkeymonkey") {
        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
        assert_eq!(branches[3], 7777);
    }
    else {
        assert!(false);
    }
}