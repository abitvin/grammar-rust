use grammar::Grammar;

#[test]
fn alter() {
    let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
    
    let f = |_, l: &str| {
        assert_eq!(l, "<AAA<BBB>CCC>");
        Ok(111)
    };

    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("alter", "(~\\\\<,<|\\\\>,>|東,AAA|💝,BBB|中,CCC)");
    grammar.map("root", "<alter>{7}", f);
    
    let compiled = grammar.compile().unwrap();
    
    if let Ok(b) = compiled.scan("root", code) {
        assert_eq!(b[0], 111);
    }
    else {
        assert!(false);
    }
}