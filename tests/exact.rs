use grammar::Grammar;

#[test]
fn exact() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey{2}", |_, _: &str| Ok(1234));

    let compiled = grammar.compile().unwrap();

    if let Ok(_) = compiled.scan("root", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(_) = compiled.scan("root", "monkey") {
        assert!(false);
    }
    else {
        assert!(true);
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