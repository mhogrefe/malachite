use std::str::FromStr;

use rug;

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[test]
fn test_integer_partial_eq_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(v).unwrap() == Natural::from_str(u).unwrap(),
            out
        );
        assert_eq!(
            Natural::from_str(u).unwrap() == Integer::from_str(v).unwrap(),
            out
        );
        assert_eq!(
            rug::Integer::from_str(u).unwrap() == rug::Integer::from_str(v).unwrap(),
            out
        );
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("123", "5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("1000000000000", "-1000000000000", false);
}
