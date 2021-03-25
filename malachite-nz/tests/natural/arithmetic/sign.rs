use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::natural::Natural;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Natural::from_str(s).unwrap().sign(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("1000000000000", Ordering::Greater);
}
