use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use num::BigInt;
use rug;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::integer::comparison::sign::num_sign;

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

#[test]
fn sign_properties() {
    test_properties(integers, |n| {
        let sign = n.sign();
        assert_eq!(n.partial_cmp(&(0 as Limb)), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(Integer::from(i).sign(), i.sign());
    });
}
