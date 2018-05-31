extern crate grammar;
use grammar::Grammar;

#[test]
fn none_or_many() {
    let f = |_: Vec<i32>, _: &str| {
        vec![1983, 2, 7]
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "monkey*", Some(Box::new(f)));

    if let Ok(branches) = grammar.scan("root", "") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkey") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "monkeymonkeymonkey") {
        assert_eq!(branches[0], 1983);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 7);
    }
    else {
        assert!(false);
    }
}