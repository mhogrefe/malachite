use std::str::FromStr;

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[test]
fn test_from_natural() {
    let test = |s| {
        let x = Integer::from(Natural::from_str(s).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Integer::from(&Natural::from_str(s).unwrap());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}
