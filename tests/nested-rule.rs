extern crate grammer;
use grammer::Grammer;

#[test]
fn nested_rule()
{
    let f = |_: Vec<i32>, _: &str, _: &mut bool| {
        vec![7777]
    };

    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.declare(vec!["monkey"]);    // Also testing `declare` here.
    grammer.add("test-a", "<monkey>", None);
    grammer.add("test-b", "<monkey><monkey><monkey>", None);
    grammer.add("test-c", "<monkey>+", None);
    grammer.add("test-d", "<monkey>*", None);
    grammer.add("monkey", "monkey", Some(Box::new(f)));

    if let Ok(_) = grammer.scan("test-a", "ape", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("test-a", "monkey", &mut false) {
        assert_eq!(branches[0], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-b", "monkeymonkeymonkey", &mut false) {
        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("test-c", "", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("test-c", "monkeymonkeymonkeymonkey", &mut false) {
        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
        assert_eq!(branches[3], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-d", "", &mut false) {
        assert_eq!(branches.len(), 0);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-d", "monkeymonkeymonkeymonkey", &mut false) {
        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
        assert_eq!(branches[3], 7777);
    }
    else {
        assert!(false);
    }
}