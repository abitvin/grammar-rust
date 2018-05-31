extern crate grammar;
use grammar::Grammar;

#[test]
fn literal() {
    let f = |_: Vec<i32>, _: &str| {
        vec![123, 456, 789]
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey", Some(Box::new(f)));

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 123);
        assert_eq!(branches[1], 456);
        assert_eq!(branches[2], 789);
    }
    else {
        assert!(false);
    }
}