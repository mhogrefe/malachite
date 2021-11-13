use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::CheckedInto;
use malachite_base_test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::integer_gen;
use num::{BigInt, Signed};
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();

        let abs = n.clone().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&n).abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().abs().to_string(), out);

        let abs = n.clone().unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&n).unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let x = n.clone();
        let abs = x.unsigned_abs_ref();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let mut x = n;
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

#[test]
fn abs_properties() {
    integer_gen().test_properties(|x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(bigint_to_integer(&integer_to_bigint(&x).abs()), abs);

        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(&x).abs()),
            abs
        );

        let abs_alt = (&x).abs();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        assert!(abs >= 0);
        assert_eq!(abs == x, x >= 0);
        assert_eq!((&abs).abs(), abs);

        let abs_alt = x.clone().unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(abs_alt), (&abs).checked_into());

        let abs_alt = (&x).unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(&abs_alt), abs.checked_into().as_ref());

        let internal_abs = x.unsigned_abs_ref();
        assert!(internal_abs.is_valid());
        assert_eq!(*internal_abs, abs_alt);
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            Integer::from(i).abs(),
            Integer::from(SignedDoubleLimb::from(i).abs())
        );
    });
}
