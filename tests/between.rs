extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn between()
{
    let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
        vec![1234]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("root", "monkey{2,4}", Some(Box::new(f)));

    if let Ok(_) = grammer.scan("root", "") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(_) = grammer.scan("root", "monkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkey") {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkey") {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkey") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}