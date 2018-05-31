extern crate grammar;
use grammar::Grammar;

#[test]
fn exact() {
    let f = |_: Vec<i32>, _: &str| {
        vec![1234]
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey{2}", Some(Box::new(f)));

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
        assert_eq!(branches.len(), 1);
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