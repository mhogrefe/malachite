use std::str::FromStr;

use malachite_nz_test_util::natural::arithmetic::neg::neg_num;
use num::BigUint;
use rug;

use malachite_nz::natural::Natural;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&Natural::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);
        assert_eq!(neg_num(BigUint::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("1000000000000", "-1000000000000");
    test("2147483648", "-2147483648");
}
