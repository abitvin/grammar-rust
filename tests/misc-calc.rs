use grammar::Grammar;

#[test]
fn misc_calc() {
    let mut grammar: Grammar<f64> = Grammar::new();
    grammar.add("num", "[0-9]+", Some(Box::new(|_, l| l.parse().unwrap())));
    grammar.add("brackets", "\\(<expr>\\)", None);
    grammar.add("mul", "(<num>|<brackets>)(\\*<mul>)?", Some(Box::new(|b, _| if b.len() == 1 { b[0] } else { b[0] * b[1] } )));
    grammar.add("add", "<mul>(\\+<add>)?", Some(Box::new(|b, _| if b.len() == 1 { b[0] } else { b[0] + b[1] } )));
    grammar.add("expr", "(<add>|<brackets>)", None);

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