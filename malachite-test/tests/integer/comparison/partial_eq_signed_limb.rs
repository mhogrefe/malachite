use std::str::FromStr;

use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::pairs_of_integer_and_signed;
use malachite_test::integer::comparison::partial_eq_signed_limb::num_partial_eq_signed_limb;

#[test]
fn test_partial_eq_signed_limb() {
    let test = |u, v: SignedLimb, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_signed_limb(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("123", 5, false);
    test("-123", 123, false);
    test("123", -123, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn partial_eq_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let eq = *n == i;
            assert_eq!(num_partial_eq_signed_limb(&integer_to_bigint(n), i), eq);
            assert_eq!(integer_to_rug_integer(n) == i, eq);
            assert_eq!(*n == Integer::from(i), eq);

            assert_eq!(i == *n, eq);
            assert_eq!(i == integer_to_rug_integer(n), eq);
            assert_eq!(Integer::from(i) == *n, eq);
        },
    );

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}
