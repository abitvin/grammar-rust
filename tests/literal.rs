extern crate grammer;
use grammer::Grammer;

#[test]
fn literal()
{
    let f = |_: Vec<i32>, _: &str, _: &mut bool| {
        vec![123, 456, 789]
    };

    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("root", "monkey", Some(Box::new(f)));

    if let Ok(branches) = grammer.scan("root", "monkey", &mut false) {
        assert_eq!(branches[0], 123);
        assert_eq!(branches[1], 456);
        assert_eq!(branches[2], 789);
    }
    else {
        assert!(false);
    }
}