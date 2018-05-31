extern crate grammar;
use grammar::Grammar;

#[test]
fn at_least() {
    let f = |_: Vec<i32>, _: &str| {
        vec![1234]
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey{2,}", Some(Box::new(f)));

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

    if let Ok(branches) = grammar.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }
}