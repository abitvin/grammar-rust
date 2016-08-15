extern crate grammer;
use grammer::Grammer;

#[test]
fn grammer_alter()
{
    let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
    
    let f = |_: Vec<i32>, l: &str, _: &mut bool| {
        assert_eq!(l, "<AAA<BBB>CCC>");
        vec![111, 222]
    };

    let mut grammer: Grammer<i32, bool> = Grammer::new();
    grammer.add("alter", "(~\\\\<,<|\\\\>,>|東,AAA|💝,BBB|中,CCC)", None);
    grammer.add("root", "<alter>{7}", Some(Box::new(f)));
    
    if let Ok(b) = grammer.scan("root", code, &mut false) {
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], 111);
        assert_eq!(b[1], 222);
    }
    else {
        assert!(false);
    }
}