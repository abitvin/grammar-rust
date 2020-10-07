use grammar::Grammar;

#[test]
fn at_least_one() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "monkey+", |_, _: &str| 5678);

    if let Ok(_) = grammar.scan("root", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }
}