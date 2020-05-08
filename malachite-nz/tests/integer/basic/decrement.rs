use std::str::FromStr;

use malachite_base::crement::Crementable;

use malachite_nz::integer::Integer;

#[test]
fn test_decrement() {
    let test = |u, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.decrement();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "-1");
    test("123", "122");
    test("1000000000000", "999999999999");
    test("1", "0");
    test("-123", "-124");
    test("-1000000000000", "-1000000000001");
}
