extern crate grammar;
use grammar::Grammar;

#[test]
fn at_least_one() {
    let f = |_: Vec<i32>, _: &str| 5678;

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey+", Some(Box::new(f)));

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