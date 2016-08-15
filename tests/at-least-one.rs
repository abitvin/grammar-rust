extern crate grammer;
use grammer::Grammer;

#[test]
fn at_least_one()
{
    let f = |_: Vec<i32>, _: &str, _: &mut bool| {
        vec![5678]
    };

    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("root", "monkey+", Some(Box::new(f)));

    if let Ok(_) = grammer.scan("root", "", &mut false) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    if let Ok(branches) = grammer.scan("root", "monkey", &mut false) {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "monkeymonkeymonkeymonkeymonkeymonkey", &mut false) {
        assert_eq!(branches[0], 5678);
    }
    else {
        assert!(false);
    }
}