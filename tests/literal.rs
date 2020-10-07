use grammar::Grammar;

#[test]
fn literal() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey", |_, _: &str| 123);

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 123);
    }
    else {
        assert!(false);
    }
}