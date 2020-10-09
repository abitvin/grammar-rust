use grammar::Grammar;

#[test]
fn not() {
    let mut grammar: Grammar<i32> = Grammar::new();
    grammar.rule("not-monkey", "!monkey");
    grammar.rule("not-monkey+", "!monkey+");
    grammar.rule("not-monkey*", "!monkey*");        // This rule always fails, which is correct but impractical.
    grammar.rule("gorilla", "gorilla");
    grammar.rule("not-monkey-then-gorilla", "!monkey<gorilla>");
    grammar.rule("not-monkey+-then-gorilla", "!monkey+<gorilla>");
    grammar.rule("not-monkey*-then-gorilla", "!monkey*<gorilla>");

    let compiled = grammar.compile().unwrap();
    
    assert!(compiled.scan("not-monkey", "").is_ok());
    assert!(compiled.scan("not-monkey+", "").is_ok());
    assert!(compiled.scan("not-monkey*", "").is_err());

    assert!(compiled.scan("not-monkey-then-gorilla", "gorilla").is_ok());
    assert!(compiled.scan("not-monkey+-then-gorilla", "gorilla").is_ok());
    assert!(compiled.scan("not-monkey*-then-gorilla", "gorilla").is_err());
    assert!(compiled.scan("not-monkey*-then-gorilla", "monkeygorilla").is_err());
}