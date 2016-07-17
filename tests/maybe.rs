extern crate grammer;
use grammer::Grammer;
use grammer::NoShared;

#[test]
fn maybe()
{
    let f = |_: Vec<i32>, _: &str, _: &mut NoShared| {
        vec![1940, 3, 10]
    };

    let mut grammer: Grammer<i32> = Grammer::new();
    //TODO grammer.add("root", "Chuck\\ Norris\\ counted\\ to\\ infinity\\ -\\ twice?", Some(Box::new(f)));
    grammer.add("root", "twice?", Some(Box::new(f)));

    if let Ok(branches) = grammer.scan("root", "") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    //TODO if let Ok(branches) = grammer.scan("root", "Chuck Norris counted to infinity - twice") {
    if let Ok(branches) = grammer.scan("root", "twice") {
        assert_eq!(branches[0], 1940);
        assert_eq!(branches[1], 3);
        assert_eq!(branches[2], 10);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = grammer.scan("root", "twicetwice") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
