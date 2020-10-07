use grammar::Grammar;

#[test]
fn misc_calc() {
    let mut grammar: Grammar<f64> = Grammar::new();
    grammar.map("num", "[0-9]+", |_, l| l.parse().unwrap());
    grammar.rule("brackets", "\\(<expr>\\)");
    grammar.map("mul", "(<num>|<brackets>)(\\*<mul>)?", |b, _| if b.len() == 1 { b[0] } else { b[0] * b[1] } );
    grammar.map("add", "<mul>(\\+<add>)?", |b, _| if b.len() == 1 { b[0] } else { b[0] + b[1] } );
    grammar.rule("expr", "(<add>|<brackets>)");

    if let Ok(branches) = grammar.scan("expr", "2*(3*4*5)") {
        assert_eq!(branches[0], 120f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("expr", "2*(3+4)*5") {
        assert_eq!(branches[0], 70f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammar.scan("expr", "((2+3*4+5))") {
        assert_eq!(branches[0], 19f64);
    }
    else {
        assert!(false);
    }
}