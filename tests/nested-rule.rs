extern crate grammer;
use grammer::Grammer;

#[test]
fn nested_rule() {
    let f = |_: Vec<i32>, _: &str| {
        vec![7777]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("test-a", "<monkey>", None);
    grammer.add("test-b", "<monkey><monkey><monkey>", None);
    grammer.add("test-c", "<monkey>+", None);
    grammer.add("test-d", "<monkey>*", None);
    grammer.add("monkey", "monkey", Some(Box::new(f)));

    if let Ok(_) = grammer.scan("test-a", "ape") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("test-a", "monkey") {
        assert_eq!(branches[0], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-b", "monkeymonkeymonkey") {
        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("test-c", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("test-c", "monkeymonkeymonkeymonkey") {
        assert_eq!(branches.len(), 4);
        assert_eq!(branches[0], 7777);
        assert_eq!(branches[1], 7777);
        assert_eq!(branches[2], 7777);
        assert_eq!(branches[3], 7777);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-d", "") {
        assert_eq!(branches.len(), 0);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("test-d", "monkeymonkeymonkeymonkey") {
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