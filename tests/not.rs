use grammar::Grammar;

#[test]
fn not() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("not-monkey", "!monkey");
    grammar.rule("not-monkeys", "!monkey*");
    grammar.rule("gorilla", "gorilla");
    grammar.rule("not-monkey-but-gorilla", "!monkey<gorilla>");
    grammar.rule("not-monkeys-but-gorilla", "!monkey*<gorilla>");

    if let Ok(_) = grammar.scan("not-monkey", "") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    /* TODO Fix me
    if let Ok(branches) = grammar.scan("not-monkeys", "") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    */
    if let Ok(_) = grammar.scan("not-monkey-but-gorilla", "gorilla") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    /* TODO Fix me
    if let Ok(branches) = grammar.scan("not-monkeys-but-gorilla", "gorilla") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    */
}