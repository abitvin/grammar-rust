use grammar::Grammar;

#[test]
fn at_least_one() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey+", |_, _: &str| 5678);

    let compiled = grammar.compile().unwrap();

    if let Ok(_) = compiled.scan("root", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = compiled.scan("root", "monkey") {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = compiled.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }
}