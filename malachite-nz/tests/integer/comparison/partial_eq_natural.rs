use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rug;
use std::str::FromStr;

#[test]
fn test_integer_partial_eq_natural() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(v == u, out);
        assert_eq!(
            rug::Integer::from_str(s).unwrap() == rug::Integer::from_str(t).unwrap(),
            out
        );
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("-123", "123", false);
    test("123", "5", false);
    test("1000000000000", "123", false);
    test("123", "1000000000000", false);
    test("1000000000000", "1000000000000", true);
    test("-1000000000000", "1000000000000", false);
}
