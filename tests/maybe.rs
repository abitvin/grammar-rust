use grammar::Grammar;

#[test]
fn maybe() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "Maybe?", |_, _: &str| 1234);

    if let Ok(branches) = grammar.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("root", "Maybe") {
        assert_eq!(branches[0], 1234);
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
