extern crate grammer;
use grammer::Grammer;

#[test]
fn between()
{
    let f = |_: Vec<i32>, _: &str, _: &mut bool| {
        vec![1234]
    };

    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("root", "monkey{2,4}", Some(Box::new(f)));

    if let Ok(_) = grammer.scan("root", "", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(_) = grammer.scan("root", "monkey", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkey", &mut false) {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkey", &mut false) {
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], 1234);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkey", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}