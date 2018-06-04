extern crate grammar;
use grammar::Grammar;

#[test]
fn alter() {
    let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
    
    let f = |_: Vec<i32>, l: &str| {
        assert_eq!(l, "<AAA<BBB>CCC>");
        111
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("alter", "(~\\\\<,<|\\\\>,>|東,AAA|💝,BBB|中,CCC)", None);
    grammar.add("root", "<alter>{7}", Some(Box::new(f)));
    
    if let Ok(b) = grammar.scan("root", code) {
        assert_eq!(b[0], 111);
    }
    else {
        assert!(false);
    }
}