use cli::{cat, echo, ls};

#[test]
pub fn test_echo() {
    assert_eq!(echo(vec!["a", "b", "c"]), String::from("a b c"));
}

#[test]
pub fn test_echo2() {
    assert_eq!(echo(vec![]), String::new());
}