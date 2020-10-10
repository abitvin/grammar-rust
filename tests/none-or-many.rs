use grammar::Grammar;

#[test]
fn none_or_many() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey*", |_, _: &str| Ok(1234));

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

    if let Ok(branches) = compiled.scan("root", "monkeymonkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }
}