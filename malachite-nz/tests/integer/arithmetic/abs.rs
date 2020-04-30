use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use num::{BigInt, Signed};

use malachite_nz::integer::Integer;

#[test]
fn test_abs() {
    let test = |s, out| {
        let abs = Integer::from_str(s).unwrap().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&Integer::from_str(s).unwrap()).abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().abs().to_string(), out);

        let abs = Integer::from_str(s).unwrap().unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&Integer::from_str(s).unwrap()).unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let x = Integer::from_str(s).unwrap();
        let abs = x.unsigned_abs_ref();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let mut x = Integer::from_str(s).unwrap();
        x.abs_assign();
        assert!(abs.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "1000000000000");
    test("3000000000", "3000000000");
    test("-3000000000", "3000000000");
    test("-2147483648", "2147483648");
}
