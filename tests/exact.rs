use grammar::Grammar;

#[test]
fn exact() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey{2}", |_, _: &str| 1234);

    if let Ok(_) = grammar.scan("root", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(_) = grammar.scan("root", "monkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammar.scan("root", "monkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammar.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}