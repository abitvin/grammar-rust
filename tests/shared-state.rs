extern crate grammer;
use grammer::Grammer;

#[test]
fn shared_state()
{
    let mut grammer: Grammer<bool, i32> = Grammer::new();
    grammer.add("one", "one", Some(Box::new(|_, _, shared| { *shared += 1; vec![] } )));
    grammer.add("two", "two", Some(Box::new(|_, _, shared| { *shared += 2; vec![] } )));
    grammer.add("three", "three", Some(Box::new(|_, _, shared| { *shared += 3; vec![] } )));
    grammer.add("root", "(<one>|<two>|<three>)*", None);
    
    let mut shared = 100;

    if let Ok(branches) = grammer.scan("root", "onetwoone", &mut shared) {
        assert_eq!(branches.len(), 0);
        assert_eq!(shared, 104);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "threethreethreetwotwoone", &mut shared) {
        assert_eq!(branches.len(), 0);
        assert_eq!(shared, 118);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = grammer.scan("root", "", &mut shared) {
        assert_eq!(branches.len(), 0);
        assert_eq!(shared, 118);
    }
    else {
        assert!(false);
    }
}