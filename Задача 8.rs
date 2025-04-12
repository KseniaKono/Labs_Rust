use std::cmp::Ordering;

fn min<T: Ord>(a: T, b: T) -> T {
    match a.cmp(&b) {
        Ordering::Greater => b,
        _ => a,
    }
}

#[test]
fn test_numbers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn test_chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn test_strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
