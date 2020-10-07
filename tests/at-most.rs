use grammar::Grammar;

#[test]
fn at_most() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey{,2}", |_, _: &str| 1234);

    let compiled = grammar.compile().unwrap();

    if let Ok(branches) = compiled.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = compiled.scan("root", "monkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = compiled.scan("root", "monkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = compiled.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
