extern crate grammer;
use grammer::Grammer;

#[test]
fn not() {
    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("not-monkey", "!monkey", None);
    grammer.add("not-monkeys", "!monkey*", None);
    grammer.add("gorilla", "gorilla", None);
    grammer.add("not-monkey-but-gorilla", "!monkey<gorilla>", None);
    grammer.add("not-monkeys-but-gorilla", "!monkey*<gorilla>", None);

    if let Ok(branches) = grammer.scan("not-monkey", "") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    /* TODO Fix me
    if let Ok(branches) = grammer.scan("not-monkeys", "") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    */
    if let Ok(branches) = grammer.scan("not-monkey-but-gorilla", "gorilla") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    /* TODO Fix me
    if let Ok(branches) = grammer.scan("not-monkeys-but-gorilla", "gorilla") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    */
}