extern crate grammer;
use grammer::Grammer;

#[test]
fn maybe() {
    let f = |_: Vec<i32>, _: &str| {
        vec![1940, 3, 10]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    grammer.add("root", "Maybe?", Some(Box::new(f)));

    if let Ok(branches) = grammer.scan("root", "") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "Maybe") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("root", "MaybeMaybe") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
