use grammar::Grammar;

#[test]
fn literal() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey", |_, _: &str| Ok(123));

    let compiled = grammar.compile().unwrap();

    if let Ok(branches) = compiled.scan("root", "monkey") {
        assert_eq!(branches[0], 123);
    }
    else {
        assert!(false);
    }
}