use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz_test_util::integer::arithmetic::sign::num_sign;
use num::BigInt;
use rug;

use malachite_nz::integer::Integer;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Integer::from_str(s).unwrap().sign(), out);
        assert_eq!(num_sign(&BigInt::from_str(s).unwrap()), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("-123", Ordering::Less);
    test("1000000000000", Ordering::Greater);
    test("-1000000000000", Ordering::Less);
}
