extern crate grammer;
use grammer::Grammer;

#[test]
fn misc_calc()
{
    let mut grammer: Grammer<f64, bool> = Grammer::new();
    grammer.declare(vec!["add", "expr", "mul"]);
    grammer.add("num", "[0-9]+", Some(Box::new(|_, l, _| vec![l.parse().unwrap()])));
    grammer.add("brackets", "\\(<expr>\\)", None);
    grammer.add("mul", "(<num>|<brackets>)(\\*<mul>)?", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b[0] * b[1]] } )));
    grammer.add("add", "<mul>(\\+<add>)?", Some(Box::new(|b, _, _| if b.len() == 1 { b } else { vec![b[0] + b[1]] } )));
    grammer.add("expr", "(<add>|<brackets>)", None);

    if let Ok(branches) = grammer.scan("expr", "2*(3*4*5)", &mut false) {
        assert_eq!(branches[0], 120f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("expr", "2*(3+4)*5", &mut false) {
        assert_eq!(branches[0], 70f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("expr", "((2+3*4+5))", &mut false) {
        assert_eq!(branches[0], 19f64);
    }
    else {
        assert!(false);
    }
}