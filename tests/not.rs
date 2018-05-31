extern crate grammar;
use grammar::Grammar;

#[test]
fn not() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.add("not-monkey", "!monkey", None);
    grammar.add("not-monkeys", "!monkey*", None);
    grammar.add("gorilla", "gorilla", None);
    grammar.add("not-monkey-but-gorilla", "!monkey<gorilla>", None);
    grammar.add("not-monkeys-but-gorilla", "!monkey*<gorilla>", None);

    if let Ok(branches) = grammar.scan("not-monkey", "") {
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
    if let Ok(branches) = grammar.scan("not-monkey-but-gorilla", "gorilla") {
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