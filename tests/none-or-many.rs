use grammar::Grammar;

#[test]
fn none_or_many() {
    let f = |_, _: &str| 1234;

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey*", f);

    if let Ok(branches) = grammar.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkeymonkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }
}