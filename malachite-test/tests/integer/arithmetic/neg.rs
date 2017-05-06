use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -native::Integer::from_str(s).unwrap();
        assert_eq!(neg.to_string(), out);
        assert!(neg.is_valid());

        let neg = -gmp::Integer::from_str(s).unwrap();
        assert_eq!(neg.to_string(), out);
        assert!(neg.is_valid());

        assert_eq!((-num::BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rugint::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
}
