use grammar::Grammar;

#[test]
fn maybe() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.map("root", "Maybe?", |_, _: &str| Ok(1234));

    let compiled = grammar.compile().unwrap();

    if let Ok(branches) = compiled.scan("root", "") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = compiled.scan("root", "Maybe") {
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = compiled.scan("root", "MaybeMaybe") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
