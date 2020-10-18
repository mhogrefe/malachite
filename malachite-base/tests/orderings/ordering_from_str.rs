use std::cmp::Ordering;

use malachite_base::orderings::ordering_from_str;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(ordering_from_str(s), out);
    };
    test("Equal", Some(Ordering::Equal));
    test("Less", Some(Ordering::Less));
    test("Greater", Some(Ordering::Greater));

    test("", None);
    test("abc", None);
    test("Lesser", None);
}
