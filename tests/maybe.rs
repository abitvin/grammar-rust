extern crate grammar;
use grammar::Grammar;

#[test]
fn maybe() {
    let f = |_: Vec<i32>, _: &str| {
        vec![1940, 3, 10]
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("root", "Maybe?", Some(Box::new(f)));

    if let Ok(branches) = grammar.scan("root", "") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "Maybe") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammar.scan("root", "MaybeMaybe") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
