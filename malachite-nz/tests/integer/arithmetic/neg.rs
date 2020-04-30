use std::str::FromStr;

use malachite_base::num::arithmetic::traits::NegAssign;
use num::BigInt;

use malachite_nz::integer::Integer;

#[test]
fn test_neg() {
    let test = |s, out| {
        let neg = -Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&Integer::from_str(s).unwrap();
        assert!(neg.is_valid());
        assert_eq!(neg.to_string(), out);

        assert_eq!((-BigInt::from_str(s).unwrap()).to_string(), out);
        assert_eq!((-rug::Integer::from_str(s).unwrap()).to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
        x.neg_assign();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "-123");
    test("-123", "123");
    test("1000000000000", "-1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
    test("2147483648", "-2147483648");
}
